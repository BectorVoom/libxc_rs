//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 403/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk403<F: Float>(t1963: F, t40: F, t539: F, t592: F, t544: F, t559: F, t712: F, t171: F, t1: F, t558: F, t598: F, t110: F, t518: F, t84: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t1964 = t40 * t1963;
    let t1966 = F::new(8.0) * t539 * t592;
    let t1968 = F::new(8.0) * t544 * t592;
    let t1969 = t539 * t559;
    let t1970 = F::new(8.0) * t1969;
    let t1972 = t712 * t712;
    let t1974 = t171 * t171;
    let t1975 = F::new(1.0) / t1974;
    let t1979 = t558 * t1;
    let t1980 = t1979 * t598;
    let t1981 = F::new(0.36623110073506319882e-3) * t1980;
    let t1983 = t518 * t110 * t84;
    (t1964, t1966, t1968, t1970, t1972, t1974, t1975, t1979, t1981, t1983)
}
