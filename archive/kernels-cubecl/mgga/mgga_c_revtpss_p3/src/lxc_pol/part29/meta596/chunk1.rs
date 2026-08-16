//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 2004/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2004<F: Float>(t103117: F, t7064: F, t103103: F, t103114: F, t103119: F, t103122: F, t103130: F, t14662: F, t2061: F, t231: F, t25391: F, t26515: F, t26550: F, t27199: F, t27353: F, t2771: F, t51525: F, t7070: F, t7076: F, t8006: F, t93118: F, t95762: F, t95766: F, t95768: F, t99289: F) -> F {
    let t103136 = F::cast_from(0.25702851531048074406e-1_f64) * t7064 * t103117;
    let t103137 = -t103103 - F::cast_from(0.77108554593144223218e-1_f64) * t95762 + F::cast_from(0.10408353825846239354e2_f64) * t7070 * t93118 * t8006 * t2771 + F::cast_from(0.4336814094102599731e0_f64) * t27353 * t26550 * t51525 + F::cast_from(0.51405703062096148812e-1_f64) * t95766 + F::cast_from(0.43368140941025997312e-1_f64) * t95768 - F::cast_from(0.96373646535613327357e-2_f64) * t103114 + t103119 + F::cast_from(0.4336814094102599731e0_f64) * t27199 * t26515 + F::cast_from(0.22849835011101738147e-2_f64) * t103122 + F::cast_from(0.4336814094102599731e0_f64) * t7070 * t7076 * t2061 * t14662 * t231 + F::cast_from(0.24093411633903331839e-3_f64) * t103130 - F::cast_from(0.8673628188205199462e0_f64) * t25391 * t26550 * t99289 - t103136;
    t103137
}
