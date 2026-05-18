//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1245/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1245<F: Float>(t322: F, t12005: F, t1338: F, t40851: F, t3678: F, t6755: F, t1348: F, t6767: F, t11145: F, t11157: F, t12002: F, t12009: F, t2438: F, t31929: F, t3413: F, t352: F, t3675: F, t37199: F, t37218: F, t6751: F, t8481: F, t855: F, t9760: F) -> F {
    let t332 = F::new(0.25e1) < t322;
    let t41028 = t1338 * t12005;
    let t41033 = piecewise3::<f64>(t332, t40851, F::new(0.0));
    let t41039 = t6755 * t3678;
    let t41042 = t1348 * t12005;
    let t41047 = t6767 * t3678;
    let t41054 = -F::new(0.21e1) * t37218 * t3675 - F::new(0.42e1) * t11145 * t9760 - F::new(0.21e1) * t3413 * t31929 - F::new(0.42e1) * t41028 * t2438 - F::new(0.21e1) * t12002 * t6751 - F::new(0.105e1) * t855 * t41033 * t352 - F::new(0.63e1) * t12009 * t8481 - F::new(0.945e1) * t41039 * t8481 - F::new(0.315e1) * t41042 * t2438 - F::new(0.1575e1) * t12009 * t6751 - F::new(0.23625e1) * t41047 * t8481 - F::new(0.1575e1) * t37199 * t3675 - F::new(0.315e1) * t11157 * t9760;
    t41054
}
