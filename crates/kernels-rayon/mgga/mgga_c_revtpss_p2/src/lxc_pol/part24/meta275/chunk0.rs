//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1048/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1048(t177: f64, t5941: f64, t762: f64, t1553: f64, t73: f64, t2475: f64, t5966: f64, t5962: f64, t853: f64, t221: f64, t2485: f64, t6017: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18562 = t5941 * t177;
    let t18563 = t18562 * t762;
    let t18592 = t1553 * t73;
    let t18599 = t2475 * t5966;
    let t18608 = t853 * t5962;
    let t18622 = t2485 * t221 * t6017;
    (t18562, t18563, t18592, t18599, t18608, t18622)
}
