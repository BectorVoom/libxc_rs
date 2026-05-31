//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1950/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1950<F: Float>(t28093: F, t7349: F, t26169: F, t7702: F, t28640: F, t6954: F, t1923: F, t28089: F, t7348: F, t26205: F, t101360: F, t2048: F, t25150: F, t26172: F, t7352: F, t7964: F, t95297: F, t95314: F, t95320: F) -> F {
    let t101899 = F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t28093 * t7349;
    let t101901 = F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t7702 * t26169;
    let t101903 = F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t6954 * t28640;
    let t101906 = F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t1923 * t7348 * t28089;
    let t101907 = t7702 * t26205;
    let t101919 = -F::cast_from(160.0_f64) / F::cast_from(9.0_f64) * t95297 - t101899 - t101901 - t101903 - t101906 + F::cast_from(88.0_f64) / F::cast_from(27.0_f64) * t101907 - F::cast_from(352.0_f64) / F::cast_from(27.0_f64) * t95314 - F::cast_from(80.0_f64) / F::cast_from(3.0_f64) * t95320 + t101360 * t2048 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t28093 * t7352 + t7702 * t26172 / F::cast_from(3.0_f64) + t25150 * t7964 / F::cast_from(3.0_f64);
    t101919
}
