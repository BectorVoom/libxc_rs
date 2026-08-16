//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta288 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1056;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1057;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta288(t3777: f64, t3798: f64, t1354: f64, t1307: f64, t3719: f64, t3870: f64, t820: f64, t12189: f64, t1329: f64, t3726: f64, t3770: f64, t119: f64, t12012: f64, t210: f64, t12211: f64, t3766: f64, t12156: f64, t1358: f64, t3774: f64, t1333: f64, t3862: f64, t10022: f64, t248: f64, t557: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12300, t12301, t12303, t12305, t12308, t12310, t12313) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1056(t3777, t3798, t1354, t1307, t3719, t3870, t820, t12189, t1329, t3726, t3770, t119, t12012, t210);
        let (t12317, t12320, t12323, t12325, t12328) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1057(t12211, t3766, t119, t12156, t210, t1358, t3774, t1333, t3862, t10022, t248, t557);
    (t12300, t12301, t12303, t12305, t12308, t12310, t12313, t12317, t12320, t12323, t12325, t12328)
}
