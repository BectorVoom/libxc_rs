//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 338/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk338<F: Float>(t1267: F, t1243: F, t1250: F, t1253: F, t1258: F, t1262: F, t295: F, t299: F, t305: F, t803: F, t807: F, t815: F, t818: F) -> (F, F, F) {
    let t1268 = F::new(11.0) / F::new(9.0) * t1267;
    let t1269 = F::new(40.0) / F::new(9.0) * t1243 * t299 - F::new(50.0) / F::new(9.0) * t803 * t807 + F::new(10.0) / F::new(9.0) * t295 * t1250 + F::new(5.0) / F::new(3.0) * t295 * t1253 + F::new(10.0) / F::new(9.0) * t305 * t1258 + F::new(5.0) / F::new(3.0) * t305 * t1262 - t1268;
    let t1271 = t815 * t818;
    (t1268, t1269, t1271)
}
