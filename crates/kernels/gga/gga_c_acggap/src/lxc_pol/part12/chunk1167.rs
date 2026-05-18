//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1167/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1167<F: Float>(t34284: F, t34286: F, t34293: F, t34295: F, t34297: F, t34305: F, t34307: F, t34315: F, t34317: F, t30321: F, t30325: F, t34281: F, t34291: F, t34301: F, t34309: F, t34311: F, t34313: F, t34321: F) -> F {
    let t37008 = F::new(0.16006300097412701803e-1) * t34284;
    let t37009 = F::new(0.90035438047946447644e-2) * t34286;
    let t37012 = F::new(0.32012600194825403606e-1) * t34293;
    let t37013 = F::new(0.32012600194825403606e-1) * t34295;
    let t37014 = F::new(0.21437009059034868486e-2) * t34297;
    let t37016 = F::new(0.12579236915841660828e-2) * t34305;
    let t37017 = F::new(0.12862205435420921092e-1) * t34307;
    let t37021 = F::new(0.17149607247227894789e-2) * t34315;
    let t37022 = F::new(0.25724410870841842184e-2) * t34317;
    let t37024 = F::new(0.66040993808168719343e-1) * t34281 - F::new(0.85748036236139473944e-3) * t30321 - t37008 + t37009 + F::new(0.37737710747524982482e-2) * t30325 - F::new(0.18868855373762491241e-2) * t34291 + t37012 + t37013 - t37014 + F::new(0.94344276868812456207e-3) * t34301 + t37016 + t37017 + F::new(0.80031500487063509014e-2) * t34309 + F::new(0.51448821741683684366e-2) * t34311 - F::new(0.34299214494455789578e-2) * t34313 + t37021 + t37022 - F::new(0.10718504529517434243e-2) * t34321;
    t37024
}
