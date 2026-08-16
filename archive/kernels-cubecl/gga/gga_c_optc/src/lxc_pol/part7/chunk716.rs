//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 716/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk716<F: Float>(t601: F, t6825: F, t6735: F, t87: F, t40: F, t1906: F, t591: F, t2045: F, t559: F, t1979: F, t1983: F, t518: F, t622: F, t84: F) -> (F, F, F, F, F, F, F, F) {
    let t6827 = F::cast_from(0.35089340384731224426e1_f64) * t601 * t6825;
    let t6828 = t6735 * t87;
    let t6829 = t40 * t6828;
    let t6830 = t1906 * t591;
    let t6831 = t40 * t6830;
    let t6832 = F::cast_from(3.0_f64) * t6831;
    let t6833 = t2045 * t559;
    let t6834 = F::cast_from(36.0_f64) * t6833;
    let t6835 = t1979 * t1983;
    let t6836 = F::cast_from(0.73246220147012639764e-3_f64) * t6835;
    let t6838 = t518 * t622 * t84;
    (t6827, t6828, t6829, t6830, t6832, t6834, t6836, t6838)
}
