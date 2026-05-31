//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1199/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1199<F: Float>(t35549: F, t35552: F, t35556: F, t35560: F, t31363: F, t31374: F, t31382: F, t31386: F, t32795: F, t32796: F, t32799: F, t32800: F, t32803: F, t35545: F, t35564: F, t35567: F, t35569: F, t35573: F) -> F {
    let t37605 = F::cast_from(0.12579236915841660828e-2_f64) * t35549;
    let t37606 = F::cast_from(0.18868855373762491241e-2_f64) * t35552;
    let t37607 = F::cast_from(0.12579236915841660828e-2_f64) * t35556;
    let t37610 = F::cast_from(35.0_f64) / F::cast_from(216.0_f64) * t35560;
    let t37617 = F::cast_from(0.34299214494455789578e-2_f64) * t35545 - t37605 + t37606 - t37607 - F::cast_from(0.31448092289604152068e-2_f64) * t31363 + F::cast_from(0.3361875e0_f64) * t31374 + t37610 - t32795 - t32796 + F::cast_from(13.0_f64) / F::cast_from(24.0_f64) * t31382 + F::cast_from(0.17149607247227894789e-2_f64) * t31386 + t32799 - t32800 - t32803 + F::cast_from(0.27439371595564631662e-1_f64) * t35564 + F::cast_from(0.42874018118069736972e-3_f64) * t35567 + F::cast_from(0.62896184579208304138e-2_f64) * t35569 - F::cast_from(0.62896184579208304138e-2_f64) * t35573;
    t37617
}
