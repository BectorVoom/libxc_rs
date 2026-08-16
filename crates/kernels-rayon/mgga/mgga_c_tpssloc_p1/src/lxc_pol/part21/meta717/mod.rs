//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta717 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2558;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2559;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta717(t3070: f64, t43198: f64, t4578: f64, t4574: f64, t14192: f64, t2960: f64, t10510: f64, t4641: f64, t1020: f64, t1616: f64, t248: f64, t43216: f64, t10489: f64, t4644: f64, t10898: f64, t4630: f64, t10882: f64, t48569: f64, t13961: f64, t3109: f64, t13542: f64, t2970: f64, t973: f64, t13546: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t50147, t50169, t50172, t50174, t50181) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2558(t3070, t43198, t4578, t4574, t14192, t2960, t10510, t4641, t1020, t1616, t248, t43216);
        let (t50183, t50189, t50193, t50229, t50242, t50250) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2559(t10489, t4644, t10898, t4630, t10882, t48569, t13961, t3109, t13542, t2970, t973, t13546);
    (t50147, t50169, t50172, t50174, t50181, t50183, t50189, t50193, t50229, t50242, t50250)
}
