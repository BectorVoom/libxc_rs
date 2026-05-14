//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 815/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk815<F: Float>(t333: F, t3724: F, t1167: F, t3676: F, t317: F, t3675: F, t305: F, t1164: F, t2869: F) -> (F, F, F, F) {
    let t12888 = 1.0 / t3724 / t333;
    let t12905 = t1167 * t3676;
    let t12909 = 1.0 / t3675 / t317;
    let t12910 = t305 * t12909;
    let t12929 = t2869 * t1164;
    (t12888, t12905, t12910, t12929)
}
