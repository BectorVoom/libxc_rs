//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1284/1429 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1284<F: Float>(t116: F, t1968: F, t204: F, t34159: F, t169: F, t3081: F, t35194: F, t11412: F, t26447: F, t27624: F, t11431: F, t27754: F) -> (F, F, F, F) {
    let t35316 = t116 * t1968 * t34159 * t204;
    let t35319 = t169 * t35194 * t3081;
    let t35323 = t169 * t11412 * t26447 * t27624;
    let t35325 = t11431 * t27754;
    (t35316, t35319, t35323, t35325)
}
