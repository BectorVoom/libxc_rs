//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 367/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk367<F: Float>(t1611: F, t2241: F, t2283: F, t2339: F, t2347: F, t240: F, t555: F, t650: F, sigma2: F) -> (F, F) {
    let t2351 = t2241 - t2283 + t240 * (-t1611 * t2347 + t2339 * t555 - t2241 + t2283);
    let t2355 = F::new(1.0) / t650;
    let t2356 = sigma2 * t2355;
    (t2351, t2356)
}
