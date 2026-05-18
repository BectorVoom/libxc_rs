//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1067/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1067<F: Float>(t31718: F, t953: F, t1378: F, t3902: F, t930: F, t1405: F, t3843: F, t940: F, t1392: F, t7947: F, t1434: F, t999: F) -> (F, F, F, F, F) {
    let t32576 = t953 * t31718;
    let t32722 = t930 * t3902 * t1378;
    let t33398 = t940 * t3843 * t1405;
    let t33492 = t1392 * t7947;
    let t33574 = t999 * t3902 * t1434;
    (t32576, t32722, t33398, t33492, t33574)
}
