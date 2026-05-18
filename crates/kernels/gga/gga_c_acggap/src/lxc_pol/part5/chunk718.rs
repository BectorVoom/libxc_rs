//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 718/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk718<F: Float>(t1182: F, t372: F, t1181: F, t1552: F, t1532: F, t3169: F, t1008: F, t1446: F, t1451: F, t1180: F, t3743: F, t3745: F, t3747: F, t3750: F, t3752: F, t3757: F, t3762: F, t3766: F, t3768: F, t3772: F, t3773: F, t3777: F, t3778: F, t3782: F, t3783: F, t3787: F) -> (F, F, F, F, F, F) {
    let t5207 = t1182 * t372;
    let t5209 = t1181 * t1552 * t5207;
    let t5213 = t1181 * t1532 * t3169;
    let t5222 = F::new(0.34299214494455789578e-2) * t1008 * t1446;
    let t5224 = F::new(0.17149607247227894789e-2) * t1008 * t1451;
    let t5225 = F::new(0.80031500487063509016e-2) * t3743 - F::new(0.80031500487063509016e-2) * t3745 + F::new(0.16006300097412701803e-1) * t3747 - F::new(0.16006300097412701803e-1) * t3750 + F::new(0.80031500487063509016e-2) * t3752 - F::new(0.85748036236139473944e-3) * t3757 + F::new(0.85748036236139473944e-3) * t3762 - F::new(0.11337795902333997111e-1) * t3766 + F::new(0.17149607247227894789e-2) * t1180 * t5209 - F::new(0.85748036236139473944e-3) * t1180 * t5213 + F::new(0.42874018118069736972e-2) * t3768 + t3772 - F::new(0.60023625365297631762e-2) * t3773 + t3777 - F::new(0.20007875121765877254e-1) * t3778 + t3782 + F::new(0.20007875121765877254e-2) * t3783 - F::new(0.42874018118069736972e-3) * t3787 - t5222 - t5224;
    (t5207, t5209, t5213, t5222, t5224, t5225)
}
