//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 2035/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2035(t12725: f64, t12734: f64, t1983: f64, t2040: f64, t2096: f64, t22574: f64, t22578: f64, t22607: f64, t2314: f64, t23953: f64, t24175: f64, t24432: f64, t24442: f64, t24990: f64, t24995: f64, t26558: f64, t26878: f64, t26898: f64, t27163: f64, t3652: f64, t4028: f64, t45632: f64, t5361: f64, t55934: f64, t652: f64, t6876: f64, t7050: f64, t7166: f64, t7685: f64, t7796: f64, t7801: f64, t7806: f64, t7940: f64, t7941: f64, t86672: f64, t91565: f64, t91603: f64, t91695: f64, t9348: f64) -> f64 {
    let t94103 = 6.0_f64 * t1983 * t24175 * t24990 + t86672 * t2096 - 2.0_f64 * t4028 * t24442 - 2.0_f64 * t9348 * t7806 - 2.0_f64 * t652 * t3652 * t7801 - t1983 * t7940 * t22578 - 2.0_f64 * t45632 * t2040 - 4.0_f64 * t12734 * t7796 - 4.0_f64 * t2314 * t27163 - 6.0_f64 * t24995 * t24432 * t91695 + 12.0_f64 * t22574 * t26558 * t91565 - 4.0_f64 * t55934 * t2040 - 4.0_f64 * t12725 * t7050 + 2.0_f64 * t7166 * t5361 - 3.0_f64 * t22574 * t24432 * t91603 + t22607 * t7941 + 6.0_f64 * t6876 * t26898 + 3.0_f64 * t7685 * t23953 - 2.0_f64 * t6876 * t26878;
    t94103
}
