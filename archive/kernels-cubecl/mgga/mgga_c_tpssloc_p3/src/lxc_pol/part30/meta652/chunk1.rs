//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2067/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2067<F: Float>(t1539: F, t6746: F, t82655: F, t14220: F, t7581: F, t25555: F, t82822: F, t25529: F, t6680: F, t1920: F, t2966: F, t7614: F) -> (F, F, F, F, F) {
    let t89395 = t82655 * t1539 * t6746;
    let t89399 = t82655 * t7581 * t14220;
    let t89421 = F::cast_from(0.18277045187202515961e-2_f64) * t82822 * t25555;
    let t89429 = F::cast_from(0.14621636149762012769e-1_f64) * t6680 * t25529;
    let t89431 = t1920 * t2966 * t7614;
    (t89395, t89399, t89421, t89429, t89431)
}
