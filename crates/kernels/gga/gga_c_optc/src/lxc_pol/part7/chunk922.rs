//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 922/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk922<F: Float>(t6760: F, t737: F, t732: F, t188: F, t1911: F, t6680: F, t6686: F, t1916: F, t6674: F, t6602: F, t758: F, t1917: F, t2238: F, t1912: F, t2229: F, t2234: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t21939 = t737 * t6760;
    let t21943 = t732 * t6760;
    let t21946 = t188 * t6680 * t1911;
    let t21948 = t737 * t6686;
    let t21950 = t732 * t6686;
    let t21953 = t188 * t1916 * t6674;
    let t21957 = t6602 * t758;
    let t21959 = t2238 * t1917;
    let t21962 = t2229 * t1912;
    let t21964 = t2234 * t1912;
    (t21939, t21943, t21946, t21948, t21950, t21953, t21957, t21959, t21962, t21964)
}
