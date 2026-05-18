//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1249/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1249<F: Float>(t12351: F, t1348: F, t3774: F, t6767: F, t1338: F, t6755: F, t11302: F, t11314: F, t11993: F, t12348: F, t12355: F, t2438: F, t31929: F, t3549: F, t3556: F, t3675: F, t38953: F, t38966: F, t6751: F, t8481: F, t9760: F) -> F {
    let t42101 = t1348 * t12351;
    let t42106 = t6767 * t3774;
    let t42121 = t1338 * t12351;
    let t42128 = t6755 * t3774;
    let t42131 = -F::new(0.126e2) * t11314 * t11993 - F::new(0.315e1) * t42101 * t2438 - F::new(0.1575e1) * t12355 * t6751 - F::new(0.23625e1) * t42106 * t8481 - F::new(0.1575e1) * t38966 * t3675 - F::new(0.315e1) * t11314 * t9760 - F::new(0.1575e1) * t3556 * t31929 - F::new(0.21e1) * t38953 * t3675 - F::new(0.42e1) * t11302 * t9760 - F::new(0.21e1) * t3549 * t31929 - F::new(0.42e1) * t42121 * t2438 - F::new(0.21e1) * t12348 * t6751 - F::new(0.63e1) * t12355 * t8481 - F::new(0.945e1) * t42128 * t8481;
    t42131
}
