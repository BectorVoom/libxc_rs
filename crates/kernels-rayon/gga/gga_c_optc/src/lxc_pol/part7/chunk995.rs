//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 995/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk995(t6760: f64, t737: f64, t732: f64, t188: f64, t1911: f64, t6680: f64, t6686: f64, t1916: f64, t6674: f64, t6602: f64, t758: f64, t1917: f64, t2238: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t21939 = t737 * t6760;
    let t21943 = t732 * t6760;
    let t21946 = t188 * t6680 * t1911;
    let t21948 = t737 * t6686;
    let t21950 = t732 * t6686;
    let t21953 = t188 * t1916 * t6674;
    let t21957 = t6602 * t758;
    let t21959 = t2238 * t1917;
    (t21939, t21943, t21946, t21948, t21950, t21953, t21957, t21959)
}
