//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1265/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1265(t10141: f64, t1819: f64, t555: f64, t10145: f64, t10107: f64, t1804: f64, t6214: f64, t3819: f64, t6160: f64, t3029: f64, t584: f64, t1196: f64, t1198: f64, t1200: f64, t1202: f64, t1204: f64, t1206: f64, t1208: f64, t1880: f64, t3855: f64, t3857: f64, t3859: f64, t3861: f64, t3863: f64, t3865: f64, t3867: f64, t3869: f64, t3871: f64, t3873: f64, t8036: f64) -> (f64, f64, f64, f64, f64) {
    let t27099 = t555 * t1819 * t10141;
    let t27102 = t555 * t1819 * t10145;
    let t27105 = t1804 * t6214 * t10107;
    let t27120 = t555 * t6160 * t3819;
    let t27139 = t584 * t3029;
    let t27176 = -t1200 * t27139 / 20.0_f64 + t1202 * t27139 / 288.0_f64 - t1204 * t27139 / 5376.0_f64 + t1206 * t27139 / 122880.0_f64 - t1208 * t27139 / 3317760.0_f64 + t8036 * t27139 / 103219200.0_f64 - 8.0_f64 / 3.0_f64 * t1196 * t27139 + t1198 * t27139 / 2.0_f64 + 9.0_f64 / 80.0_f64 * t3855 * t1880 - t3857 * t1880 / 80.0_f64 - 11.0_f64 / 1152.0_f64 * t3859 * t1880 + t3861 * t1880 / 1152.0_f64 + 13.0_f64 / 21504.0_f64 * t3863 * t1880 - t3865 * t1880 / 21504.0_f64 - t3867 * t1880 / 32768.0_f64 + t3869 * t1880 / 491520.0_f64 + 17.0_f64 / 13271040.0_f64 * t3871 * t1880 - t3873 * t1880 / 13271040.0_f64;
    (t27099, t27102, t27105, t27120, t27176)
}
