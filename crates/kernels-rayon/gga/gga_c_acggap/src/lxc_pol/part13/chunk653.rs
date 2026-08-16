//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 653/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk653(t1182: f64, t372: f64, t1181: f64, t1552: f64, t1532: f64, t3169: f64, t1008: f64, t1446: f64, t1451: f64, t1180: f64, t3743: f64, t3745: f64, t3747: f64, t3750: f64, t3752: f64, t3757: f64, t3762: f64, t3766: f64, t3768: f64, t3772: f64, t3773: f64, t3777: f64, t3778: f64, t3782: f64, t3783: f64, t3787: f64) -> (f64, f64, f64, f64) {
    let t5207 = t1182 * t372;
    let t5209 = t1181 * t1552 * t5207;
    let t5213 = t1181 * t1532 * t3169;
    let t5222 = 0.34299214494455789578e-2_f64 * t1008 * t1446;
    let t5224 = 0.17149607247227894789e-2_f64 * t1008 * t1451;
    let t5225 = 0.80031500487063509016e-2_f64 * t3743 - 0.80031500487063509016e-2_f64 * t3745 + 0.16006300097412701803e-1_f64 * t3747 - 0.16006300097412701803e-1_f64 * t3750 + 0.80031500487063509016e-2_f64 * t3752 - 0.85748036236139473944e-3_f64 * t3757 + 0.85748036236139473944e-3_f64 * t3762 - 0.11337795902333997111e-1_f64 * t3766 + 0.17149607247227894789e-2_f64 * t1180 * t5209 - 0.85748036236139473944e-3_f64 * t1180 * t5213 + 0.42874018118069736972e-2_f64 * t3768 + t3772 - 0.60023625365297631762e-2_f64 * t3773 + t3777 - 0.20007875121765877254e-1_f64 * t3778 + t3782 + 0.20007875121765877254e-2_f64 * t3783 - 0.42874018118069736972e-3_f64 * t3787 - t5222 - t5224;
    (t5207, t5209, t5213, t5225)
}
