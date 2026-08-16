//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta259 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1005;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1006;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1007;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta259(t1090: f64, t3509: f64, t3578: f64, t1216: f64, t3252: f64, t3248: f64, t11642: f64, t11644: f64, t11649: f64, t11652: f64, t11655: f64, t11662: f64, t11665: f64, t11670: f64, t11674: f64, t11678: f64, t1227: f64, t3496: f64, t3506: f64, t3536: f64, t3577: f64, t3580: f64, t11677: f64, t3624: f64, t3516: f64, t3521: f64, t820: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11679, t11680, t11683, t11684, t11687, t11688, t11691) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1005(t1090, t3509, t3578, t1216, t3252, t3248, t11642, t11644, t11649, t11652, t11655, t11662, t11665, t11670, t11674, t11678, t1227, t3496, t3506, t3536, t3577, t3580);
        let t11692 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1006(t11677, t3624);
        let (t11693, t11694, t11697) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1007(t1090, t3516, t3578, t3521, t820);
    (t11679, t11680, t11683, t11684, t11687, t11688, t11691, t11692, t11693, t11694, t11697)
}
