//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1197/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1197<F: Float>(t35484: F, t35486: F, t35496: F, t35502: F, t35513: F, t35515: F, t31251: F, t31254: F, t31256: F, t31259: F, t31262: F, t31277: F, t31279: F, t35490: F, t35494: F, t35511: F, t35519: F, t35523: F) -> F {
    let t37569 = F::cast_from(0.28582678745379824648e-3_f64) * t35484;
    let t37570 = F::cast_from(0.25724410870841842184e-2_f64) * t35486;
    let t37573 = F::cast_from(0.18868855373762491241e-1_f64) * t35496;
    let t37576 = F::cast_from(0.14291339372689912324e-2_f64) * t35502;
    let t37583 = F::cast_from(0.18868855373762491241e-2_f64) * t35513;
    let t37584 = F::cast_from(0.12862205435420921092e-1_f64) * t35515;
    let t37587 = -t37569 - t37570 - F::cast_from(0.21437009059034868486e-3_f64) * t35490 + F::cast_from(0.42874018118069736972e-3_f64) * t35494 + t37573 + F::cast_from(0.62896184579208304137e-3_f64) * t31251 - F::cast_from(0.17149607247227894789e-2_f64) * t31254 - t37576 - F::cast_from(0.85748036236139473944e-3_f64) * t31256 + F::cast_from(0.78443749999999999999e0_f64) * t31259 + F::cast_from(0.52295833333333333333e0_f64) * t31262 - F::new(0.794625e0) * t31277 - F::new(0.52975e0) * t31279 + F::cast_from(0.18868855373762491241e-2_f64) * t35511 + t37583 + t37584 + F::cast_from(0.64311027177104605458e-2_f64) * t35519 - F::cast_from(0.42874018118069736972e-3_f64) * t35523;
    t37587
}
