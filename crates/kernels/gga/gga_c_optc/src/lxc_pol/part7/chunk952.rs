//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 952/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk952<F: Float>(t10195: F, t10345: F, t22508: F, t22510: F, t22513: F, t22516: F, t22522: F, t22524: F, t22526: F, t22528: F, t515: F, t534: F, t1824: F, t1827: F, t1788: F, t1791: F) -> (F, F, F, F, F) {
    let t22562 = 1.0 * t515 * (-0.21099166666666666667e1 * t22508 + 0.202552e2 * t22510 - 0.75019259259259259258e1 * t22513 + 0.6564185185185185185e1 * t22516 + 0.31003950617283950618e1 * t10195 + 0.68258333333333333335e-1 * t22522 - 0.10921333333333333333e1 * t22524 + 0.12134814814814814815e1 * t22526 + 0.10617962962962962963e1 * t22528 + 0.13388493827160493828e1 * t10345) * t534;
    let t22563 = t1824 * t1824;
    let t22566 = t1827 * t1827;
    let t22571 = t1788 * t1788;
    let t22574 = t1791 * t1791;
    (t22562, t22563, t22566, t22571, t22574)
}
