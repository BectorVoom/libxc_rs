//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 928/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk928<F: Float>(t1967: F, t8497: F, t2001: F, t4528: F, t1998: F, t4523: F, t7676: F, t8689: F, t1426: F, t535: F, t598: F, t7470: F, t34281: F, t34283: F, t34284: F, t34286: F, t34288: F, t34291: F, t34293: F, t34296: F, t34298: F, t34301: F, t34305: F, t34308: F, t34309: F) -> (F,) {
    let t34311 = t1967 * t8497;
    let t34312 = 0.25724410870841842184e-2 * t34311;
    let t34313 = t2001 * t4528;
    let t34315 = t1998 * t4523;
    let t34317 = t7676 * t8689;
    let t34321 = t598 * t1426 * t535 * t7470;
    let t34323 = 0.33020496904084359671e-1 * t34281 - t34283 - 0.80031500487063509014e-2 * t34284 + 0.45017719023973223821e-2 * t34286 + t34288 - 0.94344276868812456204e-3 * t34291 + 0.16006300097412701803e-1 * t34293 + t34296 - t34298 + 0.47172138434406228102e-3 * t34301 + 0.62896184579208304136e-3 * t34305 + t34308 + 0.40015750243531754508e-2 * t34309 + t34312 - 0.17149607247227894789e-2 * t34313 + 0.85748036236139473944e-3 * t34315 + 0.12862205435420921092e-2 * t34317 - 0.53592522647587171215e-3 * t34321;
    (t34323,)
}
