//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1038/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1038<F: Float>(t31551: F, t852: F, t1486: F, t193: F, t1091: F, t24981: F, t7062: F, t24980: F, t5408: F, t6334: F, t24976: F, t6317: F, t10248: F, t446: F, t1476: F, t5225: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t31552 = t852 * t31551;
    let t31554 = t1486 * t193 * t31552;
    let t31561 = t24981 * t7062 * t1091;
    let t31562 = t24980 * t31561;
    let t31564 = t6334 * t5408;
    let t31565 = t24976 * t31564;
    let t31566 = t6317 * t31565;
    let t31569 = t10248 * t31564;
    let t31570 = t446 * t31569;
    let t31572 = t1476 * t5225;
    (t31552, t31554, t31561, t31562, t31564, t31565, t31566, t31569, t31570, t31572)
}
