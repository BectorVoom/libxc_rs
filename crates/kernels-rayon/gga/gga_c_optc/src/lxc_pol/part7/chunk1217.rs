//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1217/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1217(t24: f64, t7398: f64, t862: f64, t22021: f64, t2633: f64, t23548: f64, t7253: f64, t22015: f64, t10913: f64, t10918: f64, t23590: f64, t23662: f64, t25072: f64, t25075: f64, t25078: f64, t25087: f64, t25091: f64, t25095: f64, t25107: f64, t25112: f64, t2583: f64, t2603: f64, t2606: f64, t2623: f64, t2640: f64, t2722: f64, t322: f64, t3608: f64, t7399: f64, t7410: f64, t7460: f64, t7481: f64, t7852: f64, t7859: f64, t7867: f64, t893: f64, t899: f64) -> (f64, f64, f64) {
    let t25115 = t862 * t24 * t7398;
    let t25117 = t2633 * t22021;
    let t25121 = t7253 * t23548;
    let t25122 = t25121 * t22015;
    let t25133 = -0.1794440248262568288e1_f64 * t25072 * t899 + 0.4893927949807004422e0_f64 * t25075 + 0.28977204965962526181e-1_f64 * t25078 - 0.23181763972770020946e0_f64 * t2583 * t7867 + t862 * t2722 * t23662 / 8.0_f64 - 0.43465807448943789272e-1_f64 * t893 * t25087 + 0.47242254414539272975e4_f64 * t25091 + 0.10866451862235947318e0_f64 * t893 * t25095 - 0.3863627328795003491e-1_f64 * t2583 * t7852 - 0.17171677016866682182e0_f64 * t2583 * t7859 - t862 * t3608 * t23590 / 6.0_f64 - 11.0_f64 / 27.0_f64 * t7410 * t2603 + 2.0_f64 / 27.0_f64 * t25107 - 2.0_f64 / 9.0_f64 * t2623 * t7399 + t25112 / 108.0_f64 + t25115 / 36.0_f64 + t862 * t322 * t25117 / 72.0_f64 - 7.0_f64 / 54.0_f64 * t862 * t322 * t25122 + 0.1420012659563261767e0_f64 * t2640 * t7460 * t10913 + 0.23666877659387696117e0_f64 * t2640 * t7481 * t2606 * t10918;
    (t25117, t25122, t25133)
}
