//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta676 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2266;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2267;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta676<F: Float>(t2314: F, t26003: F, t1874: F, t90381: F, t1983: F, t2019: F, t55169: F, t510: F, t652: F, t86604: F, t26114: F, t6535: F, t26179: F, t25994: F, t12823: F, t7461: F, t25980: F, t4034: F, t12813: F, t89: F, t6525: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t91724, t91726, t91730, t91735, t91737) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2266::<F>(t2314, t26003, t1874, t90381, t1983, t2019, t55169, t510, t652, t86604, t26114, t6535);
        let (t91739, t91747, t91749, t91752, t91755, t91757) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2267::<F>(t26179, t6535, t2314, t25994, t12823, t7461, t25980, t4034, t12813, t89, t1874, t6525);
    (t91724, t91726, t91730, t91735, t91737, t91739, t91747, t91749, t91752, t91755, t91757)
}
