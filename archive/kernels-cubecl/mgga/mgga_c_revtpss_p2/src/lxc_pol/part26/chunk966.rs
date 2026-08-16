//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 966/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk966<F: Float>(t12292: F, t12296: F, t12297: F, t12299: F, t12301: F, t12303: F, t12307: F, t12310: F, t12314: F, t12317: F, t12320: F, t1132: F) -> (F, F) {
    let t12322 = -t12296 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t12297 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t12299 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t12301 - t12303 / F::cast_from(3.0_f64) + F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t12307 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t12310 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t12292 + F::cast_from(2.0_f64) * t12314 + F::cast_from(2.0_f64) * t12317 + t12320 / F::cast_from(3.0_f64);
    let t12323 = t1132 * t12322;
    (t12322, t12323)
}
