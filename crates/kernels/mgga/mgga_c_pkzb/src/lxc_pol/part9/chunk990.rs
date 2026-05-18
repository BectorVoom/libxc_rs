//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 990/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk990<F: Float>(t12: F, t7549: F, t7896: F, t1429: F, t4803: F, t1151: F, t1153: F, t2159: F, t2163: F, t3000: F, t3005: F, t318: F, t319: F, t808: F, t810: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F, F) {
    let t84 = t12 <= zeta_threshold;
    let t203 = rho0 <= dens_threshold || t84;
    let t7897 = t7549 + t7896;
    let t7906 = F::new(2.0) * t1429;
    let t7907 = F::new(6.0) * t4803;
    let t7908 = t7906 - t7907;
    let t7909 = piecewise3::<f64>(t84, F::new(0.0), t7908);
    let t7913 = piecewise3::<f64>(t203, F::new(0.0), t7897 * t319 / F::new(2.0) + t3000 * t810 + t1151 * t2163 / F::new(2.0) + t2159 * t1153 / F::new(2.0) + t808 * t3005 + t318 * t7909 / F::new(2.0));
    (t7897, t7908, t7909, t7913)
}
