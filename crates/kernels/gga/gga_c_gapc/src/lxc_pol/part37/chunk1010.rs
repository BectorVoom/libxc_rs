//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1010/1445 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1010<F: Float>(t11625: F, t11626: F, t2212: F, t2268: F, t3738: F, t10346: F, t2208: F, t6201: F, t800: F, t3649: F, t760: F, t3739: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11627 = t11625 * t11626;
    let t11629 = t2268 * t2212;
    let t11630 = t3738 * t11629;
    let t11632 = t10346 * t2208;
    let t11633 = t800 * t6201;
    let t11634 = t11632 * t11633;
    let t11636 = t3649 * t760;
    let t11637 = t11636 * t2208;
    let t11638 = t11637 * t3739;
    (t11627, t11629, t11630, t11632, t11633, t11634, t11636, t11637, t11638)
}
