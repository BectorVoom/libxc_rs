//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1076/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1076(t137: f64, t5506: f64, t1426: f64, t368: f64, t598: f64, t33997: f64, t34011: f64, t34014: f64, t34024: f64, t34028: f64, t34030: f64, t34032: f64, t34036: f64, t34038: f64, t36898: f64, t36911: f64, t36914: f64, t38909: f64, t38912: f64, t38914: f64, t38916: f64, t38920: f64) -> (f64, f64) {
    let t38922 = t137 * t5506;
    let t38925 = t598 * t1426 * t368 * t38922;
    let t38927 = t33997 + 0.94344276868812456204e-3_f64 * t38909 + t36898 - 0.83861579438944405513e-2_f64 * t34011 + t34014 - 0.85748036236139473944e-3_f64 * t38912 + t34024 + t34028 + t34030 - t34032 - t36911 - t34036 - t34038 - t36914 - 0.53592522647587171215e-3_f64 * t38914 + 0.20007875121765877254e-2_f64 * t38916 - 0.53592522647587171215e-3_f64 * t38920 - 0.53592522647587171215e-3_f64 * t38925;
    (t38922, t38927)
}
