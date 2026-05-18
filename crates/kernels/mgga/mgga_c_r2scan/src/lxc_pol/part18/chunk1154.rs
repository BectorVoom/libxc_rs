//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1154/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1154<F: Float>(t322: F, t42547: F, t12692: F, t1338: F, t1348: F, t10533: F, t11145: F, t11157: F, t12002: F, t12009: F, t12683: F, t2438: F, t3413: F, t3420: F, t352: F, t35220: F, t3675: F, t37204: F, t37223: F, t41028: F, t41042: F, t855: F, t9760: F) -> F {
    let t332 = F::new(0.25e1) < t322;
    let t42775 = piecewise3::<f64>(t332, t42547, F::new(0.0));
    let t42791 = t1338 * t12692;
    let t42794 = t1348 * t12692;
    let t42807 = -F::new(0.105e1) * t855 * t42775 * t352 - F::new(0.63e1) * t11157 * t12683 - F::new(0.42e1) * t41028 * t3675 - F::new(0.42e1) * t12002 * t9760 - F::new(0.945e1) * t37223 * t12683 - F::new(0.21e1) * t11145 * t10533 - F::new(0.21e1) * t3413 * t35220 - F::new(0.21e1) * t42791 * t2438 - F::new(0.1575e1) * t42794 * t2438 - F::new(0.315e1) * t41042 * t3675 - F::new(0.315e1) * t12009 * t9760 - F::new(0.1575e1) * t11157 * t10533 - F::new(0.1575e1) * t3420 * t35220 - F::new(0.23625e1) * t37204 * t12683;
    t42807
}
