//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1027/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1027(t22321: f64, t22336: f64, t59: f64, t40: f64, t87: f64, t1963: f64, t2045: f64, t1872: f64, t2042: f64, t110: f64, t1811: f64, t1820: f64, t1826: f64, t1828: f64, t1842: f64, t1847: f64, t1850: f64, t1860: f64, t1864: f64, t1868: f64, t209: f64, t21874: f64, t21878: f64, t21887: f64, t21891: f64, t21895: f64, t21899: f64, t21903: f64, t21907: f64, t22277: f64, t22281: f64, t22285: f64, t571: f64, t6382: f64, t6420: f64, t6493: f64) -> (f64, f64, f64, f64, f64) {
    let t22338 = (t22321 + t22336) * t59;
    let t22340 = t40 * t22338 * t87;
    let t22341 = t2045 * t1963;
    let t22342 = 72.0_f64 * t22341;
    let t22344 = 120.0_f64 * t2042 * t1872;
    let t22366 = t21874 + t21878 - t21887 - t21891 - t21895 + t21899 + t21903 - 0.86748647062252193713e-1_f64 * t209 * t110 * t1847 * t1850 + 0.43374323531126096856e-1_f64 * t209 * t6493 * t1860 + 0.1284251895870376528e1_f64 * t209 * t110 * t1864 * t1868 - 0.21687161765563048428e-1_f64 * t209 * t1842 * t6420 + t21907 - t22277 + 36.0_f64 * t1826 * t1811 * t1820 - t22281 - t22285 + 0.1286587327114827919e3_f64 * t1826 * t6382 * t1828 * t571;
    (t22338, t22340, t22342, t22344, t22366)
}
