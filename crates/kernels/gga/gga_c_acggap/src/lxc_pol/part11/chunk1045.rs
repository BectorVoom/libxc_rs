//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1045/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1045<F: Float>(t1967: F, t8561: F, t30340: F, t30349: F, t30353: F, t30355: F, t30362: F, t34327: F, t34330: F, t34332: F, t34333: F, t34336: F, t34338: F, t34339: F, t34341: F, t34343: F, t34348: F, t34349: F) -> F {
    let t34351 = t1967 * t8561;
    let t34352 = F::cast_from(0.37737710747524982482e-2_f64) * t34351;
    let t34357 = F::cast_from(0.114609375e-1_f64) * t34327 + F::new(0.22921875e-1) * t34330 - t34332 - t34333 + t30340 + F::cast_from(0.31448092289604152068e-3_f64) * t34336 + t34338 + t34339 + t34341 - F::cast_from(0.10718504529517434243e-3_f64) * t34343 - t34348 - F::cast_from(0.37737710747524982482e-2_f64) * t34349 + t34352 - F::cast_from(0.94344276868812456204e-3_f64) * t30349 + F::cast_from(0.10482697429868050689e-3_f64) * t30353 - F::cast_from(0.10718504529517434243e-3_f64) * t30355 - F::cast_from(0.7145669686344956162e-4_f64) * t30362;
    t34357
}
