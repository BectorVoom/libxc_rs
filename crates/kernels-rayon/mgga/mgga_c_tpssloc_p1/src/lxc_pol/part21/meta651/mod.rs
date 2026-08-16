//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta651 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2447;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2448;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta651(t1015: f64, t10472: f64, t42559: f64, t10870: f64, t3048: f64, t204: f64, t376: f64, t1020: f64, t1023: f64, t248: f64, t10510: f64, t3109: f64, t3082: f64, t3094: f64, t1032: f64, t10375: f64, t370: f64, t374: f64, t9697: f64, t10473: f64, t361: f64, t363: f64, t42342: f64, t42345: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t43211, t43214, t43216, t43219, t43221) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2447(t1015, t10472, t42559, t10870, t3048, t204, t376, t1020, t1023, t248, t10510, t3109);
        let (t43228, t43248, t43253, t43288, t43291) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2448(t3082, t3094, t1032, t10375, t370, t374, t376, t9697, t10473, t361, t363, t42342, t42345);
    (t43211, t43214, t43216, t43219, t43221, t43228, t43248, t43253, t43288, t43291)
}
