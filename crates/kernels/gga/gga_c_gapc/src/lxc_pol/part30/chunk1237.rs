//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 1237/1331 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk1237<F: Float>(t11208: F, t11210: F, t5248: F, t102: F, t125: F, t190: F, t13853: F, t35381: F, t11214: F, t11217: F, t4050: F, t423: F) -> (F, F, F, F) {
    let t35466 = t11208 * t11210 * t5248;
    let t35469 = t102 * t125 * t190;
    let t35471 = t35381 * t35469 * t13853;
    let t35475 = t11214 * t423 * t4050 * t11217;
    (t35466, t35469, t35471, t35475)
}
