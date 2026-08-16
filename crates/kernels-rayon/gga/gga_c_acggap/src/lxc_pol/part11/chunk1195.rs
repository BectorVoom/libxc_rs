//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1195/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1195(t30248: f64, t542: f64, t1967: f64, t8855: f64, t31773: f64, t8916: f64, t7447: f64, t8920: f64, t2001: f64, t4355: f64, t31840: f64, t31843: f64, t31845: f64, t31847: f64, t31851: f64, t31855: f64, t31857: f64, t36332: f64, t36333: f64, t36335: f64, t36340: f64, t36344: f64, t36347: f64) -> f64 {
    let t36349 = t30248 * t542;
    let t36351 = t1967 * t8855;
    let t36352 = 0.12862205435420921092e-2_f64 * t36351;
    let t36353 = t31773 * t8916;
    let t36354 = 0.3361875e0_f64 * t36353;
    let t36355 = t7447 * t8920;
    let t36356 = 0.16809375e0_f64 * t36355;
    let t36358 = t2001 * t4355;
    let t36361 = -t36332 + 0.64311027177104605458e-2_f64 * t36333 - 0.34299214494455789578e-2_f64 * t36335 - t31840 - 0.10718504529517434243e-3_f64 * t31843 + 0.18868855373762491241e-2_f64 * t31845 - 0.28303283060643736861e-2_f64 * t31847 + t36340 + 0.7862023072401038017e-3_f64 * t31851 - 0.31448092289604152068e-2_f64 * t36344 + 0.47172138434406228102e-2_f64 * t36347 - 0.22675591804667994221e-1_f64 * t36349 - t36352 + t36354 + t36356 + 0.34299214494455789577e-2_f64 * t31855 - 0.51448821741683684367e-1_f64 * t36358 + 0.68598428988911579156e-2_f64 * t31857;
    t36361
}
