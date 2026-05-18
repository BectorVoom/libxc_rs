//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 618/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk618<F: Float>(t7627: F, t7662: F, t7625: F, t7629: F, t7631: F, t7636: F, t7639: F, t7643: F, t7646: F, t7649: F, t7651: F, t7654: F, t7656: F, t7658: F, t7660: F, t7664: F) -> (F, F, F) {
    let t8143 = F::new(0.97567895348519921633e-1) * t7627;
    let t8156 = F::new(0.12981128458281457309e-2) * t7662;
    let t8158 = -F::new(0.42483693136193860285e-2) * t7625 - t8143 + F::new(0.68186654135613354324e-2) * t7629 - F::new(0.90915538847484472432e-2) * t7631 + F::new(0.13637330827122670865e-1) * t7636 - F::new(0.36366215538993788972e-1) * t7639 + F::new(0.45457769423742236216e-1) * t7643 + F::new(0.48488287385325051964e-1) * t7646 + F::new(0.9072038638458063915e-3) * t7649 - F::new(0.9676841214355268176e-3) * t7651 + F::new(0.16934472125121719308e-2) * t7654 + F::new(0.11289648083414479539e-2) * t7656 + F::new(0.11974241701863808564e0) * t7658 - F::new(0.19957069503106347607e-1) * t7660 - t8156 - F::new(0.26552308210121162678e-2) * t7664;
    (t8143, t8156, t8158)
}
