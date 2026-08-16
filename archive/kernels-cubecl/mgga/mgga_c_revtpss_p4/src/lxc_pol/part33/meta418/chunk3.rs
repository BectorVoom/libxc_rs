//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1491/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1491<F: Float>(t5962: F, t853: F, t775: F, t18392: F, t832: F, t1553: F, t1555: F, t18586: F, t18592: F, t18600: F, t18603: F, t227: F, t229: F, t4409: F, t4415: F, t4417: F, t4420: F, t6006: F, t6010: F, t6013: F, t830: F, t833: F) -> F {
    let t18608 = t853 * t5962;
    let t18609 = t18608 * t775;
    let t18612 = t832 * t18392;
    let t18615 = F::cast_from(6.0_f64) * t1553 * t4420 + F::cast_from(6.0_f64) * t1555 * t4409 - t18586 * t229 - F::cast_from(24.0_f64) * t18592 * t4417 + F::cast_from(60.0_f64) * t18600 * t4415 - F::cast_from(24.0_f64) * t18603 * t4415 - F::cast_from(12.0_f64) * t18609 * t4415 + F::cast_from(3.0_f64) * t18612 * t227 + F::cast_from(3.0_f64) * t6006 * t833 - F::cast_from(12.0_f64) * t6010 * t830 + F::cast_from(3.0_f64) * t6013 * t830;
    t18615
}
