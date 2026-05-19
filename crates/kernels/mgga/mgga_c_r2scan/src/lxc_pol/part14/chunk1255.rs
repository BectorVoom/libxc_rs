//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1255/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1255<F: Float>(t37393: F, t37401: F, t37419: F, t37423: F, t39059: F, t39061: F, t39062: F, t39064: F, t41258: F, t41261: F, t41263: F, t41265: F, t41270: F, t41273: F, t41276: F) -> F {
    let t42182 = -t41258 - t41261 + t41263 - t41265 - F::cast_from(0.17347588262831798123e-3_f64) * t37393 - t39059 + F::cast_from(0.18446557979282192535e-2_f64) * t37401 + t39061 + t39062 - t39064 + t41270 + F::cast_from(0.59620292925746722032e-2_f64) * t37419 + t41273 + F::cast_from(0.1440846329149835838e-2_f64) * t37423 - t41276;
    t42182
}
