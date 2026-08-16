//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 2004/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2004(t103117: f64, t7064: f64, t103103: f64, t103114: f64, t103119: f64, t103122: f64, t103130: f64, t14662: f64, t2061: f64, t231: f64, t25391: f64, t26515: f64, t26550: f64, t27199: f64, t27353: f64, t2771: f64, t51525: f64, t7070: f64, t7076: f64, t8006: f64, t93118: f64, t95762: f64, t95766: f64, t95768: f64, t99289: f64) -> f64 {
    let t103136 = 0.25702851531048074406e-1_f64 * t7064 * t103117;
    let t103137 = -t103103 - 0.77108554593144223218e-1_f64 * t95762 + 0.10408353825846239354e2_f64 * t7070 * t93118 * t8006 * t2771 + 0.4336814094102599731e0_f64 * t27353 * t26550 * t51525 + 0.51405703062096148812e-1_f64 * t95766 + 0.43368140941025997312e-1_f64 * t95768 - 0.96373646535613327357e-2_f64 * t103114 + t103119 + 0.4336814094102599731e0_f64 * t27199 * t26515 + 0.22849835011101738147e-2_f64 * t103122 + 0.4336814094102599731e0_f64 * t7070 * t7076 * t2061 * t14662 * t231 + 0.24093411633903331839e-3_f64 * t103130 - 0.8673628188205199462e0_f64 * t25391 * t26550 * t99289 - t103136;
    t103137
}
