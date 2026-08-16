//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta730 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2585;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2586;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta730(t3447: f64, t44584: f64, t4904: f64, t44510: f64, t14753: f64, t15402: f64, t14744: f64, t1174: f64, t135: f64, t15359: f64, t11589: f64, t15293: f64, t15382: f64, t44525: f64, t11588: f64, t4928: f64, t3451: f64, t15357: f64, t3448: f64, t14740: f64, t15419: f64, t11584: f64, t15338: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t51980, t51988, t51991, t51995, t52013, t52019) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2585(t3447, t44584, t4904, t44510, t14753, t15402, t14744, t1174, t135, t15359, t11589, t15293);
        let (t52022, t52036, t52038, t52040, t52050, t52053) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2586(t15382, t3447, t44525, t11588, t4928, t3451, t15357, t3448, t14740, t15419, t11584, t15338);
    (t51980, t51988, t51991, t51995, t52013, t52019, t52022, t52036, t52038, t52040, t52050, t52053)
}
