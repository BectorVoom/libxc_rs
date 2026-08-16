//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1042/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1042(t7676: f64, t8689: f64, t1426: f64, t535: f64, t598: f64, t7470: f64, t34281: f64, t34283: f64, t34284: f64, t34286: f64, t34288: f64, t34291: f64, t34293: f64, t34296: f64, t34298: f64, t34301: f64, t34305: f64, t34308: f64, t34309: f64, t34312: f64, t34313: f64, t34315: f64) -> f64 {
    let t34317 = t7676 * t8689;
    let t34321 = t598 * t1426 * t535 * t7470;
    let t34323 = 0.33020496904084359671e-1_f64 * t34281 - t34283 - 0.80031500487063509014e-2_f64 * t34284 + 0.45017719023973223821e-2_f64 * t34286 + t34288 - 0.94344276868812456204e-3_f64 * t34291 + 0.16006300097412701803e-1_f64 * t34293 + t34296 - t34298 + 0.47172138434406228102e-3_f64 * t34301 + 0.62896184579208304136e-3_f64 * t34305 + t34308 + 0.40015750243531754508e-2_f64 * t34309 + t34312 - 0.17149607247227894789e-2_f64 * t34313 + 0.85748036236139473944e-3_f64 * t34315 + 0.12862205435420921092e-2_f64 * t34317 - 0.53592522647587171215e-3_f64 * t34321;
    t34323
}
