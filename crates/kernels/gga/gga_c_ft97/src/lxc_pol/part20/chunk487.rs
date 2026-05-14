//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 487/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk487<F: Float>(t1466: F, t1479: F, t1506: F, t301: F, t6210: F, t6215: F, t6216: F, t6219: F, t6225: F, t6263: F, t6267: F, t6354: F, t6370: F, t6375: F, t6387: F, t6391: F, t6393: F, t830: F) -> (F,) {
    let t6399 = t6210 * t1479 / 6.0 - t6215 - t6216 * t6219 / 18.0 - t1466 * t6225 / 3.0 + t1466 * t6263 / 6.0 + t1466 * t6267 / 6.0 - t830 * t1506 - t301 * t6391 + 2.0 * t6393 - 2.0 * t6354 - 2.0 * t6370 + 4.0 * t6375 - 2.0 * t6387;
    (t6399,)
}
