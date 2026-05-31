//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1278/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1278<F: Float>(t13917: F, t14583: F, t53496: F, t11375: F, t13911: F, t13925: F, t15137: F, t2376: F, t27047: F, t34850: F, t35003: F, t35193: F, t36323: F, t4002: F, t51054: F, t53012: F, t53025: F, t53028: F, t53061: F, t54928: F, t56190: F, t56194: F, t56197: F, t56199: F, t56206: F, t814: F, t859: F, t8629: F, t892: F) -> F {
    let t56209 = t13917 * t53496 * t14583;
    let t56223 = -t56190 / F::cast_from(48.0_f64) - t56194 / F::cast_from(384.0_f64) - t53012 + t54928 - t56197 / F::cast_from(192.0_f64) + t53025 - t53028 - t11375 * t27047 * t2376 * t56199 * t814 / F::cast_from(48.0_f64) - t56206 / F::cast_from(384.0_f64) + t56209 / F::cast_from(768.0_f64) - t53061 - t35193 * t4002 / F::cast_from(96.0_f64) + t8629 * t859 * t892 * t15137 / F::cast_from(96.0_f64) + t36323 * t13911 / F::cast_from(48.0_f64) - t35003 * t51054 / F::cast_from(48.0_f64) + t34850 * t13925 / F::cast_from(96.0_f64);
    t56223
}
