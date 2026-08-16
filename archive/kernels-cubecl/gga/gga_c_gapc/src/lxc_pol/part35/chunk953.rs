//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 953/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk953<F: Float>(t10346: F, t2208: F, t6201: F, t800: F, t3649: F, t760: F, t3739: F, t2456: F, t3728: F, t1062: F, t10335: F, t3643: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11632 = t10346 * t2208;
    let t11633 = t800 * t6201;
    let t11634 = t11632 * t11633;
    let t11636 = t3649 * t760;
    let t11637 = t11636 * t2208;
    let t11638 = t11637 * t3739;
    let t11640 = t3728 * t2456;
    let t11641 = t1062 * t11640;
    let t11643 = t3643 * t10335;
    (t11632, t11633, t11634, t11636, t11637, t11638, t11640, t11641, t11643)
}
