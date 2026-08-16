//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 618/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk618(t7627: f64, t7662: f64, t7625: f64, t7629: f64, t7631: f64, t7636: f64, t7639: f64, t7643: f64, t7646: f64, t7649: f64, t7651: f64, t7654: f64, t7656: f64, t7658: f64, t7660: f64, t7664: f64) -> (f64, f64, f64) {
    let t8143 = 0.97567895348519921633e-1_f64 * t7627;
    let t8156 = 0.12981128458281457309e-2_f64 * t7662;
    let t8158 = -0.42483693136193860285e-2_f64 * t7625 - t8143 + 0.68186654135613354324e-2_f64 * t7629 - 0.90915538847484472432e-2_f64 * t7631 + 0.13637330827122670865e-1_f64 * t7636 - 0.36366215538993788972e-1_f64 * t7639 + 0.45457769423742236216e-1_f64 * t7643 + 0.48488287385325051964e-1_f64 * t7646 + 0.9072038638458063915e-3_f64 * t7649 - 0.9676841214355268176e-3_f64 * t7651 + 0.16934472125121719308e-2_f64 * t7654 + 0.11289648083414479539e-2_f64 * t7656 + 0.11974241701863808564e0_f64 * t7658 - 0.19957069503106347607e-1_f64 * t7660 - t8156 - 0.26552308210121162678e-2_f64 * t7664;
    (t8143, t8156, t8158)
}
