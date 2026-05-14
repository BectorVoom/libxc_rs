//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 930/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk930<F: Float>(t129: F, t507: F, t7585: F, t7587: F, t30546: F, t8477: F, t1967: F, t8561: F, t30340: F, t30349: F, t30353: F, t30355: F, t30362: F, t34327: F, t34330: F, t34332: F, t34333: F, t34336: F, t34338: F, t34339: F, t34341: F, t34343: F) -> (F,) {
    let t34345 = t129 * t507;
    let t34347 = t7585 * t34345 * t7587;
    let t34348 = 0.14291339372689912324e-3 * t34347;
    let t34349 = t30546 * t8477;
    let t34351 = t1967 * t8561;
    let t34352 = 0.37737710747524982482e-2 * t34351;
    let t34357 = 0.114609375e-1 * t34327 + 0.22921875e-1 * t34330 - t34332 - t34333 + t30340 + 0.31448092289604152068e-3 * t34336 + t34338 + t34339 + t34341 - 0.10718504529517434243e-3 * t34343 - t34348 - 0.37737710747524982482e-2 * t34349 + t34352 - 0.94344276868812456204e-3 * t30349 + 0.10482697429868050689e-3 * t30353 - 0.10718504529517434243e-3 * t30355 - 0.7145669686344956162e-4 * t30362;
    (t34357,)
}
