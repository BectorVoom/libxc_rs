//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1261/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1261<F: Float>(t31702: F, t31704: F, t32915: F, t36070: F, t36072: F, t36075: F, t36083: F, t36115: F, t36119: F, t36123: F, t37837: F, t37848: F, t37857: F, t37859: F, t37860: F, t37861: F, t37864: F, t40418: F) -> F {
    let t42097 = -t37837 - t37848 + t36070 - t36072 + F::cast_from(0.31448092289604152069e-3_f64) * t31702 + F::cast_from(0.41930789719472202758e-3_f64) * t31704 + t36075 + t37857 - t32915 + F::cast_from(0.85748036236139473944e-3_f64) * t36083 + F::cast_from(0.37737710747524982482e-1_f64) * t40418 + t37859 + t37860 - t37861 + t37864 - F::cast_from(0.85748036236139473944e-3_f64) * t36115 + F::cast_from(0.83861579438944405516e-3_f64) * t36119 - F::cast_from(0.12579236915841660827e-2_f64) * t36123;
    t42097
}
