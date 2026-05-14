//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 759/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk759<F: Float>(t2478: F, t3133: F, t6583: F, t30839: F, t901: F, t12445: F, t1407: F, t2293: F, t587: F, t9438: F, t9439: F, t12449: F, t7014: F, t2487: F, t9448: F, t31182: F) -> (F, F, F, F, F, F, F) {
    let t39968 = t6583 * t3133 * t2478;
    let t40007 = t30839 * t901;
    let t40009 = t1407 * t12445;
    let t40013 = t587 * t9438 * t9439 * t2293;
    let t40015 = t7014 * t12449;
    let t40019 = t2487 * t9438 * t9448 * t2293;
    let t40021 = t31182 * t901;
    (t39968, t40007, t40009, t40013, t40015, t40019, t40021)
}
