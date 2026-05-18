//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 575/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk575<F: Float>(t106: F, t1299: F, t167: F, t2106: F, t3454: F, t4668: F, t4675: F, t4723: F, t670: F, t1303: F, t1317: F, t201: F, t5: F) -> (F, F, F, F) {
    let t4727 = F::new(0.27818116767324025134e1) * t106 * t4668 * t167 - F::new(0.55636233534648050268e1) * t106 * t3454 * t1299 + F::new(0.55636233534648050268e1) * t106 * t2106 * t4675 - F::new(0.27818116767324025134e1) * t106 * t670 * t4723;
    let t4733 = t1303 * t1303;
    let t4741 = t1317 * t1317;
    let t4743 = t5 * t4741 * t201;
    (t4727, t4733, t4741, t4743)
}
