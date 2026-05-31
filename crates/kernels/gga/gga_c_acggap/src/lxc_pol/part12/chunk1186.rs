//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1186/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1186<F: Float>(t35039: F, t35041: F, t35043: F, t35051: F, t35055: F, t35070: F, t35072: F, t35074: F, t35076: F, t30916: F, t30918: F, t32635: F, t35047: F, t35059: F, t35062: F, t35065: F, t35068: F, t35080: F) -> F {
    let t37361 = F::cast_from(7.0_f64) / F::cast_from(36.0_f64) * t35039;
    let t37362 = F::cast_from(7.0_f64) / F::cast_from(36.0_f64) * t35041;
    let t37363 = F::cast_from(35.0_f64) / F::cast_from(108.0_f64) * t35043;
    let t37365 = F::cast_from(0.28582678745379824648e-3_f64) * t35051;
    let t37366 = F::cast_from(0.15724046144802076034e-2_f64) * t35055;
    let t37372 = F::cast_from(0.16809375e0_f64) * t35070;
    let t37373 = F::cast_from(0.16809375e0_f64) * t35072;
    let t37374 = F::cast_from(0.1120625e0_f64) * t35074;
    let t37375 = F::cast_from(77.0_f64) / F::cast_from(288.0_f64) * t35076;
    let t37377 = F::cast_from(0.17149607247227894789e-2_f64) * t30916 + t37361 + t37362 - t37363 - F::cast_from(0.21437009059034868486e-3_f64) * t35047 - t37365 - t37366 + F::cast_from(0.94344276868812456207e-3_f64) * t30918 - t35059 / F::cast_from(8.0_f64) - t35062 / F::cast_from(8.0_f64) - F::cast_from(0.4584375e-1_f64) * t35065 - F::cast_from(0.916875e-1_f64) * t35068 - t37372 - t37373 - t37374 - t32635 - t37375 - F::cast_from(0.7640625e-2_f64) * t35080;
    t37377
}
