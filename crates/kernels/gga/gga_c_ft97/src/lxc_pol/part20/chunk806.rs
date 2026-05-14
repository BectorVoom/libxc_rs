//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 806/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk806<F: Float>(t24981: F, t6309: F, t684: F, t24980: F, t6339: F, t681: F, t89: F, t1476: F, t7640: F, t2682: F, t193: F, t375: F, t6343: F, t10248: F, t24936: F, t446: F) -> (F, F, F, F, F, F, F, F, F) {
    let t24983 = t24981 * t6309 * t684;
    let t24984 = t24980 * t24983;
    let t24986 = t681 * t6339;
    let t24987 = t89 * t24986;
    let t24989 = t7640 * t1476;
    let t24990 = t24989 * t2682;
    let t24991 = t193 * t24990;
    let t24992 = t89 * t24991;
    let t24995 = t89 * t375 * t6343;
    let t24997 = t10248 * t24936;
    let t24998 = t446 * t24997;
    (t24983, t24984, t24987, t24989, t24990, t24992, t24995, t24997, t24998)
}
