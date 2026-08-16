//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1167/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1167(t34284: f64, t34286: f64, t34293: f64, t34295: f64, t34297: f64, t34305: f64, t34307: f64, t34315: f64, t34317: f64, t30321: f64, t30325: f64, t34281: f64, t34291: f64, t34301: f64, t34309: f64, t34311: f64, t34313: f64, t34321: f64) -> f64 {
    let t37008 = 0.16006300097412701803e-1_f64 * t34284;
    let t37009 = 0.90035438047946447644e-2_f64 * t34286;
    let t37012 = 0.32012600194825403606e-1_f64 * t34293;
    let t37013 = 0.32012600194825403606e-1_f64 * t34295;
    let t37014 = 0.21437009059034868486e-2_f64 * t34297;
    let t37016 = 0.12579236915841660828e-2_f64 * t34305;
    let t37017 = 0.12862205435420921092e-1_f64 * t34307;
    let t37021 = 0.17149607247227894789e-2_f64 * t34315;
    let t37022 = 0.25724410870841842184e-2_f64 * t34317;
    let t37024 = 0.66040993808168719343e-1_f64 * t34281 - 0.85748036236139473944e-3_f64 * t30321 - t37008 + t37009 + 0.37737710747524982482e-2_f64 * t30325 - 0.18868855373762491241e-2_f64 * t34291 + t37012 + t37013 - t37014 + 0.94344276868812456207e-3_f64 * t34301 + t37016 + t37017 + 0.80031500487063509014e-2_f64 * t34309 + 0.51448821741683684366e-2_f64 * t34311 - 0.34299214494455789578e-2_f64 * t34313 + t37021 + t37022 - 0.10718504529517434243e-2_f64 * t34321;
    t37024
}
