//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 879/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk879<F: Float>(t10211: F, t10212: F, t770: F, t791: F, t2431: F, t3197: F, t6791: F, t831: F, t1062: F, t2200: F, t2212: F, t3254: F) -> (F, F, F, F) {
    let t10213 = t10211 * t10212;
    let t10215 = t791 * t770;
    let t10216 = t3197 * t2431;
    let t10217 = t10215 * t10216;
    let t10219 = t6791 * t831;
    let t10220 = t1062 * t10219;
    let t10222 = t2200 * t2212;
    let t10223 = t3254 * t10222;
    (t10213, t10217, t10220, t10223)
}
