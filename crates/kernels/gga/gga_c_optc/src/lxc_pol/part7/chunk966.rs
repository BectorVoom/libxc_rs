//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 966/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk966<F: Float>(t1948: F, t2012: F, t630: F, t6560: F, t6804: F, t9686: F, t1928: F, t6: F, t141: F, t2087: F, t21868: F, t2080: F, t2089: F, t654: F, t6919: F, t137: F) -> (F, F, F, F, F, F, F, F) {
    let t22811 = t2012 * t1948;
    let t22815 = t630 * t6560;
    let t22819 = t9686 * t6804;
    let t22822 = t6 * t1928 * t1948;
    let t22827 = t2087 * t141 * t21868;
    let t22830 = t2080 * t2089;
    let t22832 = t654 * t6919;
    let t22834 = t137 * t137;
    (t22811, t22815, t22819, t22822, t22827, t22830, t22832, t22834)
}
