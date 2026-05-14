//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 475/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk475<F: Float>(t2999: F, t355: F, t18: F, t359: F, t89: F, t375: F, t943: F, t358: F, t942: F, t363: F, t1564: F, t446: F, t1586: F, t432: F, t28: F, t1597: F, t383: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t3000 = t2999 * t355;
    let t3001 = t359 * t18;
    let t3003 = t89 * t3000 * t3001;
    let t3006 = t89 * t375 * t943;
    let t3008 = t942 * t358;
    let t3009 = t3008 * t363;
    let t3010 = t1564 * t3009;
    let t3011 = t446 * t3010;
    let t3013 = t1586 * t942;
    let t3014 = t3013 * t432;
    let t3016 = t89 * t28 * t3014;
    let t3018 = t383 * t1597;
    (t3000, t3001, t3003, t3006, t3008, t3009, t3010, t3011, t3013, t3014, t3016, t3018)
}
