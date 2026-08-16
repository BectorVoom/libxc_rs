//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 499/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk499<F: Float>(t2770: F, t2978: F, t2775: F, t976: F, t221: F, t2965: F, t339: F, t1053: F, t386: F, t68: F) -> (F, F, F, F, F, F) {
    let t3146 = t2978 * t2770;
    let t3151 = t976 * t2775;
    let t3158 = t221 * t2965;
    let t3160 = t339 * t3158 / F::cast_from(432.0_f64);
    let t3173 = F::cast_from(1.0_f64) / t1053 / t386;
    let t3174 = t68 * t3173;
    (t3146, t3151, t3158, t3160, t3173, t3174)
}
