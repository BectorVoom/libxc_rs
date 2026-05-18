//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 893/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk893<F: Float>(t1638: F, t2651: F, t6218: F, t6392: F, t6396: F, t6400: F, t6404: F, t6408: F, t8146: F, t8147: F, t8149: F, t8151: F, t8154: F, t8158: F, t8163: F, t8167: F, t8172: F, t8178: F) -> F {
    let t8184 = -t8146 + F::new(0.63479958930231934629e-2) * t8147 + F::new(0.16262400898971305031e-3) * t8149 + F::new(0.29272321618148349056e-1) * t8151 + F::new(0.679213007128961539e-1) * t8154 + F::new(0.34930954652346593434e-1) * t8158 + t8163 + t8167 - F::new(0.43341108700271342816e-1) * t2651 * t1638 - F::new(0.2600466522016280569e0) * t6218 * t8172 - t8178 + F::new(0.97574405393827830186e-2) * t6392 - F::new(0.11643651550782197811e-1) * t6396 + F::new(0.1358426014257923078e0) * t6400 - F::new(0.58218257753910989057e-2) * t6404 + F::new(0.58544643236296698112e-1) * t6408;
    t8184
}
