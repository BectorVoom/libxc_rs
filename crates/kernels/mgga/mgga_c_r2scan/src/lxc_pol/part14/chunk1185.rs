//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1185/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1185<F: Float>(t12092: F, t12095: F, t12100: F, t12103: F, t12109: F, t12111: F, t12200: F, t12204: F, t11331: F, t11335: F, t11340: F, t11344: F, t11347: F, t11350: F, t11352: F, t11354: F) -> F {
    let t41138 = t12092 / F::cast_from(2.0_f64);
    let t41139 = F::cast_from(15.0_f64) / F::cast_from(8.0_f64) * t12095;
    let t41140 = F::cast_from(5.0_f64) / F::cast_from(8.0_f64) * t12100;
    let t41141 = F::cast_from(5.0_f64) / F::cast_from(8.0_f64) * t12103;
    let t41142 = F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t12109;
    let t41143 = t12111 / F::cast_from(2.0_f64);
    let t41144 = t12200 / F::cast_from(2.0_f64);
    let t41145 = F::cast_from(5.0_f64) / F::cast_from(8.0_f64) * t12204;
    let t41146 = -t41138 - t41139 - t41140 - t41141 + t11331 + t11335 - t11340 + t11344 + t11347 + t11350 + t11352 + t41142 + t41143 + t41144 - t41145 + t11354;
    t41146
}
