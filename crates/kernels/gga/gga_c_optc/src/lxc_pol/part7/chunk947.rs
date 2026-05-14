//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 947/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk947<F: Float>(t1963: F, t2045: F, t1872: F, t2042: F, t110: F, t1811: F, t1820: F, t1826: F, t1828: F, t1842: F, t1847: F, t1850: F, t1860: F, t1864: F, t1868: F, t209: F, t21874: F, t21878: F, t21887: F, t21891: F, t21895: F, t21899: F, t21903: F, t21907: F, t22277: F, t22281: F, t22285: F, t571: F, t6382: F, t6420: F, t6493: F) -> (F, F, F) {
    let t22341 = t2045 * t1963;
    let t22342 = 72.0 * t22341;
    let t22344 = 120.0 * t2042 * t1872;
    let t22366 = t21874 + t21878 - t21887 - t21891 - t21895 + t21899 + t21903 - 0.86748647062252193713e-1 * t209 * t110 * t1847 * t1850 + 0.43374323531126096856e-1 * t209 * t6493 * t1860 + 0.1284251895870376528e1 * t209 * t110 * t1864 * t1868 - 0.21687161765563048428e-1 * t209 * t1842 * t6420 + t21907 - t22277 + 36.0 * t1826 * t1811 * t1820 - t22281 - t22285 + 0.1286587327114827919e3 * t1826 * t6382 * t1828 * t571;
    (t22342, t22344, t22366)
}
