//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 729/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk729<F: Float>(t11604: F, t3434: F, t2210: F, t3440: F, t3439: F, t3425: F, t9419: F, t12278: F, t144: F, t1651: F, t3419: F, t1643: F, t2075: F, t920: F, t2222: F, t2221: F) -> (F, F, F, F, F, F, F) {
    let t12999 = t3434 * t11604;
    let t13000 = t2210 * t12999;
    let t13003 = t3440 * t11604;
    let t13004 = t3439 * t13003;
    let t13007 = t9419 * t3425;
    let t13010 = t144 * t12278;
    let t13013 = t3419 * t1651;
    let t13014 = t2210 * t13013;
    let t13017 = t3419 * t1643;
    let t13018 = t3439 * t13017;
    let t13021 = t920 * t2075;
    let t13022 = t2222 * t13021;
    let t13023 = t2221 * t13022;
    (t13000, t13004, t13007, t13010, t13014, t13018, t13023)
}
