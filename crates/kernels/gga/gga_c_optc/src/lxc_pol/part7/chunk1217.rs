//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1217/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1217<F: Float>(t24: F, t7398: F, t862: F, t22021: F, t2633: F, t23548: F, t7253: F, t22015: F, t10913: F, t10918: F, t23590: F, t23662: F, t25072: F, t25075: F, t25078: F, t25087: F, t25091: F, t25095: F, t25107: F, t25112: F, t2583: F, t2603: F, t2606: F, t2623: F, t2640: F, t2722: F, t322: F, t3608: F, t7399: F, t7410: F, t7460: F, t7481: F, t7852: F, t7859: F, t7867: F, t893: F, t899: F) -> (F, F, F) {
    let t25115 = t862 * t24 * t7398;
    let t25117 = t2633 * t22021;
    let t25121 = t7253 * t23548;
    let t25122 = t25121 * t22015;
    let t25133 = -F::new(0.1794440248262568288e1) * t25072 * t899 + F::new(0.4893927949807004422e0) * t25075 + F::new(0.28977204965962526181e-1) * t25078 - F::new(0.23181763972770020946e0) * t2583 * t7867 + t862 * t2722 * t23662 / F::new(8.0) - F::new(0.43465807448943789272e-1) * t893 * t25087 + F::new(0.47242254414539272975e4) * t25091 + F::new(0.10866451862235947318e0) * t893 * t25095 - F::new(0.3863627328795003491e-1) * t2583 * t7852 - F::new(0.17171677016866682182e0) * t2583 * t7859 - t862 * t3608 * t23590 / F::new(6.0) - F::new(11.0) / F::new(27.0) * t7410 * t2603 + F::new(2.0) / F::new(27.0) * t25107 - F::new(2.0) / F::new(9.0) * t2623 * t7399 + t25112 / F::new(108.0) + t25115 / F::new(36.0) + t862 * t322 * t25117 / F::new(72.0) - F::new(7.0) / F::new(54.0) * t862 * t322 * t25122 + F::new(0.1420012659563261767e0) * t2640 * t7460 * t10913 + F::new(0.23666877659387696117e0) * t2640 * t7481 * t2606 * t10918;
    (t25117, t25122, t25133)
}
