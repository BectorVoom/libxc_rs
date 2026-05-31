//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1195/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1195<F: Float>(t30248: F, t542: F, t1967: F, t8855: F, t31773: F, t8916: F, t7447: F, t8920: F, t2001: F, t4355: F, t31840: F, t31843: F, t31845: F, t31847: F, t31851: F, t31855: F, t31857: F, t36332: F, t36333: F, t36335: F, t36340: F, t36344: F, t36347: F) -> F {
    let t36349 = t30248 * t542;
    let t36351 = t1967 * t8855;
    let t36352 = F::cast_from(0.12862205435420921092e-2_f64) * t36351;
    let t36353 = t31773 * t8916;
    let t36354 = F::cast_from(0.3361875e0_f64) * t36353;
    let t36355 = t7447 * t8920;
    let t36356 = F::cast_from(0.16809375e0_f64) * t36355;
    let t36358 = t2001 * t4355;
    let t36361 = -t36332 + F::cast_from(0.64311027177104605458e-2_f64) * t36333 - F::cast_from(0.34299214494455789578e-2_f64) * t36335 - t31840 - F::cast_from(0.10718504529517434243e-3_f64) * t31843 + F::cast_from(0.18868855373762491241e-2_f64) * t31845 - F::cast_from(0.28303283060643736861e-2_f64) * t31847 + t36340 + F::cast_from(0.7862023072401038017e-3_f64) * t31851 - F::cast_from(0.31448092289604152068e-2_f64) * t36344 + F::cast_from(0.47172138434406228102e-2_f64) * t36347 - F::cast_from(0.22675591804667994221e-1_f64) * t36349 - t36352 + t36354 + t36356 + F::cast_from(0.34299214494455789577e-2_f64) * t31855 - F::cast_from(0.51448821741683684367e-1_f64) * t36358 + F::cast_from(0.68598428988911579156e-2_f64) * t31857;
    t36361
}
