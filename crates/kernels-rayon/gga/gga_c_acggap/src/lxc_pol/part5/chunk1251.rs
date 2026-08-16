//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1251/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1251(t1005: f64, t6194: f64, t1181: f64, t20311: f64, t3361: f64, t4267: f64, t1165: f64, t1426: f64, t1459: f64, t1531: f64, t1532: f64, t17663: f64, t17669: f64, t17671: f64, t17681: f64, t17683: f64, t17687: f64, t17689: f64, t17691: f64, t1782: f64, t360: f64, t418: f64, t4263: f64, t4463: f64, t6138: f64, t6263: f64, t922: f64) -> f64 {
    let t23010 = t1005 * t6194;
    let t23018 = t3361 * t1181 * t4267 * t20311;
    let t23032 = 0.16006300097412701803e-1_f64 * t17663 + 0.25724410870841842184e-1_f64 * t418 * t1426 * t1459 * t1782 * t922 - 0.85748036236139473944e-3_f64 * t23010 + 0.10289764348336736873e0_f64 * t4463 * t1165 * t6138 * t4263 - 0.13719685797782315831e-1_f64 * t23018 + 0.34299214494455789578e-2_f64 * t1531 * t1181 * t1532 * t6263 * t360 - 0.32012600194825403606e-1_f64 * t17669 - 0.16006300097412701803e-1_f64 * t17671 - 0.51448821741683684367e-2_f64 * t17681 - 0.24009450146119052704e-1_f64 * t17683 + 0.48018900292238105409e-1_f64 * t17687 - 0.48018900292238105408e-1_f64 * t17689 - 0.24009450146119052704e-1_f64 * t17691;
    t23032
}
