//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2018/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2018<F: Float>(t103290: F, t103291: F, t103292: F, t103293: F, t103294: F, t103296: F, t103301: F, t106058: F, t106061: F, t106063: F, t106065: F, t99035: F) -> F {
    let t110414 = F::cast_from(0.17149607247227894789e-1_f64) * t106058 + t103290 - t103291 - t103292 - t103293 + t103294 + t103296 - F::cast_from(0.45351183609335988441e-1_f64) * t99035 + t103301 + F::cast_from(0.11433071498151929859e-3_f64) * t106061 + F::cast_from(0.40015750243531754507e-2_f64) * t106063 - F::cast_from(0.80031500487063509015e-2_f64) * t106065;
    t110414
}
