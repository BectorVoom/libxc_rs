//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta328 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1701;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1702;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta328(t12283: f64, t3809: f64, t3777: f64, t3789: f64, t12248: f64, t236: f64, t3798: f64, t1354: f64, t12189: f64, t1329: f64, t3726: f64, t3770: f64, t12211: f64, t3766: f64, t1358: f64, t3774: f64, t1333: f64, t3862: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12284, t12286, t12289, t12300) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1701(t12283, t3809, t3777, t3789, t12248, t236, t3798);
        let (t12301, t12308, t12310, t12317, t12323, t12325) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1702(t12300, t1354, t12189, t1329, t3726, t3770, t12211, t3766, t1358, t3774, t1333, t3862);
    (t12284, t12286, t12289, t12300, t12301, t12308, t12310, t12317, t12323, t12325)
}
