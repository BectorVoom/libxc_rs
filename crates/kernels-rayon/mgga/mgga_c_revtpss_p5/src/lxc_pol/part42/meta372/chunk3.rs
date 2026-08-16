//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1213/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1213(t5962: f64, t853: f64, t775: f64, t18392: f64, t832: f64, t1553: f64, t1555: f64, t18586: f64, t18592: f64, t18600: f64, t18603: f64, t227: f64, t229: f64, t4409: f64, t4415: f64, t4417: f64, t4420: f64, t6006: f64, t6010: f64, t6013: f64, t830: f64, t833: f64) -> f64 {
    let t18608 = t853 * t5962;
    let t18609 = t18608 * t775;
    let t18612 = t832 * t18392;
    let t18615 = 6.0_f64 * t1553 * t4420 + 6.0_f64 * t1555 * t4409 - t18586 * t229 - 24.0_f64 * t18592 * t4417 + 60.0_f64 * t18600 * t4415 - 24.0_f64 * t18603 * t4415 - 12.0_f64 * t18609 * t4415 + 3.0_f64 * t18612 * t227 + 3.0_f64 * t6006 * t833 - 12.0_f64 * t6010 * t830 + 3.0_f64 * t6013 * t830;
    t18615
}
