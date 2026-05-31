//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3645/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3645<F: Float>(t3385: F, t3433: F, t6471: F, t1130: F, t20469: F, t1151: F, t20629: F, t3428: F, t3432: F, t6433: F, t3436: F, t1733: F, t58460: F) -> (F, F, F, F, F) {
    let t68946 = F::cast_from(6.0_f64) * t3433 * t6471 * t3385;
    let t68947 = t20469 * t1130;
    let t68949 = F::cast_from(2.0_f64) * t68947 * t1151;
    let t68951 = F::cast_from(1.0_f64) * t20629 * t3428;
    let t68952 = t6433 * t3432;
    let t68954 = F::cast_from(0.16081979498692535067e2_f64) * t68952 * t3436;
    let t68956 = F::cast_from(2.0_f64) * t58460 * t1733;
    (t68946, t68949, t68951, t68954, t68956)
}
