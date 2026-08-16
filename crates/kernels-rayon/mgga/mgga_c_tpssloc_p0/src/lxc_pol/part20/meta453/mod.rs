//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta453 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1906;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1907;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1908;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta453(t3359: f64, t4819: f64, t1136: f64, t3351: f64, t4823: f64, t11352: f64, t1682: f64, t3333: f64, t1155: f64, t4858: f64, t1695: f64, t3395: f64, t3377: f64, t4861: f64, t14722: f64, t14704: f64, t11137: f64, t11139: f64, t11141: f64, t11143: f64, t11444: f64, t14702: f64, t14708: f64, t14720: f64, t14728: f64, t14733: f64, t14738: f64, t14742: f64, t14746: f64, t14751: f64, t14755: f64, t1675: f64, t3331: f64, t11297: f64, t11350: f64, t11361: f64, t11365: f64, t14958: f64, t15048: f64, t3334: f64, t3357: f64, t3376: f64, t3401: f64, t436: f64, t4840: f64, t4862: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15164, t15165, t15168, t15171, t15172, t15179, t15182) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1906(t3359, t4819, t1136, t3351, t4823, t11352, t1682, t3333, t1155, t4858, t1695, t3395);
        let (t15185, t15204) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1907(t3377, t4861, t14722, t14704, t11137, t11139, t11141, t11143, t11444, t14702, t14708, t14720, t14728, t14733, t14738, t14742, t14746, t14751, t14755);
        let (t15207, t15210, t15213) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1908(t1675, t3331, t1695, t3377, t11297, t11350, t11361, t11365, t14958, t15048, t15165, t15168, t15172, t15179, t15182, t15185, t15204, t3334, t3357, t3376, t3401, t436, t4840, t4862);
    (t15164, t15165, t15168, t15171, t15172, t15179, t15182, t15185, t15204, t15207, t15210, t15213)
}
