//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta314 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1069;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1070;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1071;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1072;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta314(t21723: f64, t3315: f64, t11190: f64, t11444: f64, t14702: f64, t18203: f64, t18219: f64, t18229: f64, t21760: f64, t21764: f64, t21767: f64, t21771: f64, t21774: f64, t21778: f64, t11459: f64, t423: f64, t11310: f64, t11365: f64, t1148: f64, t15126: f64, t15136: f64, t15207: f64, t21827: f64, t21901: f64, t21907: f64, t21939: f64, t21942: f64, t21947: f64, t21952: f64, t21956: f64, t21958: f64, t21960: f64, t3357: f64, t3401: f64, t436: f64, t4835: f64, t6037: f64, t6069: f64, t6085: f64, t6088: f64, t21898: f64, t300: f64, t1763: f64, t6274: f64, t11947: f64, t193: f64, t21726: f64, t21728: f64, t21730: f64, t21732: f64, t21812: f64, t21815: f64, t21829: f64, t21832: f64, t21835: f64, t21897: f64, t336: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t21961, t21963, t21975) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1069(t21723, t3315, t11190, t11444, t14702, t18203, t18219, t18229, t21760, t21764, t21767, t21771, t21774, t21778);
        let (t21988, t21990) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1070(t11459, t14702, t18203, t18219, t18229, t21760, t21764, t21767, t21771, t21774, t21778, t423);
        let t21991 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1071(t11310, t11365, t1148, t15126, t15136, t15207, t21827, t21901, t21907, t21939, t21942, t21947, t21952, t21956, t21958, t21960, t21963, t21975, t21990, t3357, t3401, t436, t4835, t6037, t6069, t6085, t6088);
        let (t21993, t21999) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1072(t21898, t21991, t300, t1763, t6274, t11947, t193, t21726, t21728, t21730, t21732, t21812, t21815, t21829, t21832, t21835, t21897, t21901, t336);
    (t21961, t21963, t21975, t21988, t21990, t21993, t21999)
}
