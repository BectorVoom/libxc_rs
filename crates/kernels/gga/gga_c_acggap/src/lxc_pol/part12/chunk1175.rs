//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1175/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1175<F: Float>(t34570: F, t34578: F, t34590: F, t34592: F, t30638: F, t30640: F, t30645: F, t30647: F, t30649: F, t30653: F, t30655: F, t32540: F, t34582: F, t34586: F, t34595: F, t34598: F, t34601: F, t34604: F) -> F {
    let t37158 = F::cast_from(0.12862205435420921092e-1_f64) * t34570;
    let t37163 = F::cast_from(0.62896184579208304134e-2_f64) * t34578;
    let t37166 = F::cast_from(0.17149607247227894789e-2_f64) * t34590;
    let t37167 = F::new(11.0) / F::new(96.0) * t34592;
    let t37172 = -F::new(35.0) / F::new(108.0) * t30638 - F::cast_from(0.21437009059034868486e-3_f64) * t30640 - F::cast_from(0.34299214494455789578e-2_f64) * t30645 + t37158 + F::cast_from(0.25724410870841842184e-2_f64) * t30647 + F::cast_from(0.12862205435420921092e-2_f64) * t30649 - F::cast_from(0.94344276868812456207e-3_f64) * t30653 - F::cast_from(0.85748036236139473944e-3_f64) * t30655 - t32540 + t37163 - F::cast_from(0.25158473831683321655e-2_f64) * t34582 + F::cast_from(0.37737710747524982483e-2_f64) * t34586 - t37166 - t37167 + t34595 / F::new(8.0) + t34598 / F::new(16.0) + t34601 / F::new(32.0) + t34604 / F::new(64.0);
    t37172
}
