//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 841/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk841<F: Float>(t16294: F, t743: F, t1317: F, t4741: F, t201: F, t5: F, t1303: F, t4733: F, t1256: F, t12979: F, t9477: F, t13062: F) -> (F, F, F, F, F, F, F) {
    let t16295 = t743 * t16294;
    let t16298 = t4741 * t1317;
    let t16300 = t5 * t16298 * t201;
    let t16301 = t743 * t16300;
    let t16310 = t4733 * t1303;
    let t16315 = t12979 * t1256;
    let t16318 = F::new(0.35089340384731224426e1) * t9477;
    let t16319 = F::new(0.17544670192365612213e1) * t13062;
    (t16295, t16300, t16301, t16310, t16315, t16318, t16319)
}
