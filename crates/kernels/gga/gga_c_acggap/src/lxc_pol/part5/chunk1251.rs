//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1251/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1251<F: Float>(t1005: F, t6194: F, t1181: F, t20311: F, t3361: F, t4267: F, t1165: F, t1426: F, t1459: F, t1531: F, t1532: F, t17663: F, t17669: F, t17671: F, t17681: F, t17683: F, t17687: F, t17689: F, t17691: F, t1782: F, t360: F, t418: F, t4263: F, t4463: F, t6138: F, t6263: F, t922: F) -> F {
    let t23010 = t1005 * t6194;
    let t23018 = t3361 * t1181 * t4267 * t20311;
    let t23032 = F::cast_from(0.16006300097412701803e-1_f64) * t17663 + F::cast_from(0.25724410870841842184e-1_f64) * t418 * t1426 * t1459 * t1782 * t922 - F::cast_from(0.85748036236139473944e-3_f64) * t23010 + F::cast_from(0.10289764348336736873e0_f64) * t4463 * t1165 * t6138 * t4263 - F::cast_from(0.13719685797782315831e-1_f64) * t23018 + F::cast_from(0.34299214494455789578e-2_f64) * t1531 * t1181 * t1532 * t6263 * t360 - F::cast_from(0.32012600194825403606e-1_f64) * t17669 - F::cast_from(0.16006300097412701803e-1_f64) * t17671 - F::cast_from(0.51448821741683684367e-2_f64) * t17681 - F::cast_from(0.24009450146119052704e-1_f64) * t17683 + F::cast_from(0.48018900292238105409e-1_f64) * t17687 - F::cast_from(0.48018900292238105408e-1_f64) * t17689 - F::cast_from(0.24009450146119052704e-1_f64) * t17691;
    t23032
}
