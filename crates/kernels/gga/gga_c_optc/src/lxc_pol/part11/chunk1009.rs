//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1009/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1009<F: Float>(t1026: F, t99: F, t1872: F, t2042: F, t110: F, t1811: F, t1820: F, t1826: F, t1828: F, t1842: F, t1847: F, t1850: F, t1860: F, t1864: F, t1868: F, t209: F, t21874: F, t21878: F, t21887: F, t21891: F, t21895: F, t21899: F, t21903: F, t21907: F, t22277: F, t22281: F, t22285: F, t571: F, t6382: F, t6420: F, t6493: F) -> (F, F, F) {
    let t22323 = F::cast_from(1.0_f64) / t99 / t1026;
    let t22344 = F::cast_from(120.0_f64) * t2042 * t1872;
    let t22366 = t21874 + t21878 - t21887 - t21891 - t21895 + t21899 + t21903 - F::cast_from(0.86748647062252193713e-1_f64) * t209 * t110 * t1847 * t1850 + F::cast_from(0.43374323531126096856e-1_f64) * t209 * t6493 * t1860 + F::cast_from(0.1284251895870376528e1_f64) * t209 * t110 * t1864 * t1868 - F::cast_from(0.21687161765563048428e-1_f64) * t209 * t1842 * t6420 + t21907 - t22277 + F::cast_from(36.0_f64) * t1826 * t1811 * t1820 - t22281 - t22285 + F::cast_from(0.1286587327114827919e3_f64) * t1826 * t6382 * t1828 * t571;
    (t22323, t22344, t22366)
}
