//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1006/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1006<F: Float>(t188: F, t1912: F, t193: F, t195: F, t1956: F, t197: F, t201: F, t21868: F, t21962: F, t21964: F, t21968: F, t21970: F, t21973: F, t21975: F, t21977: F, t21981: F, t21988: F, t21991: F, t21995: F, t21998: F, t22001: F, t22004: F, t22052: F, t2238: F, t3575: F, t5: F, t6668: F, t743: F, t750: F) -> F {
    let t22063 = F::new(3.0) * t21962 + F::new(6.0) * t21964 + F::new(3.0) * t2238 * t1912 + F::new(140.0) / F::new(3.0) * t21968 + F::new(140.0) / F::new(3.0) * t21970 + F::new(70.0) / F::new(3.0) * t21973 - F::new(14.0) * t21975 - F::new(28.0) * t21977 - F::new(1820.0) / F::new(27.0) * t21981 + F::new(3.0) * t2238 * t1956 + t188 * t743 * t5 * (-t21988 + F::new(123200.0) / F::new(243.0) * t21991 - F::new(4400.0) / F::new(27.0) * t21995 - F::new(4400.0) / F::new(27.0) * t21998 + F::new(800.0) / F::new(9.0) * t22001 + F::new(800.0) / F::new(27.0) * t22004 - F::new(25.0) / F::new(3.0) * t193 * t195 * t21868 * t197 - F::new(100.0) / F::new(9.0) * t193 * t3575 * t6668 - F::new(25.0) / F::new(9.0) * t193 * t750 * t197 * t22052) * t201 / F::new(2.0);
    t22063
}
