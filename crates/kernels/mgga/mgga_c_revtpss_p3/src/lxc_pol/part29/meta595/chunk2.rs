//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1997/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1997<F: Float>(t102951: F, t25411: F, t102928: F, t25387: F, t28404: F, t689: F, t25431: F, t28384: F, t1558: F, t25391: F, t28425: F, t95551: F, t95553: F, t95556: F, t95562: F, t95567: F, t95569: F, t95572: F, t95576: F, t99155: F) -> (F, F) {
    let t102956 = F::cast_from(0.25702851531048074406e-1_f64) * t25411 * t102951;
    let t102964 = F::cast_from(0.51405703062096148812e-1_f64) * t25387 * t102928;
    let t102967 = t28404 * t689;
    let t102969 = F::cast_from(0.14456046980341999104e-1_f64) * t25431 * t102967;
    let t102971 = F::cast_from(0.25702851531048074406e-1_f64) * t25411 * t102967;
    let t102972 = t28384 * t689;
    let t102974 = F::cast_from(0.14456046980341999104e-1_f64) * t25431 * t102972;
    let t102977 = t102956 - F::cast_from(0.19274729307122665471e-1_f64) * t95551 + F::cast_from(0.34694512752820797848e1_f64) * t25391 * t28425 * t1558 * t99155 - F::cast_from(0.28912093960683998208e-1_f64) * t95553 + t102964 + F::cast_from(0.54878743191129263322e-2_f64) * t95556 - F::cast_from(0.13009920719177044025e-2_f64) * t95562 - t102969 + t102971 - t102974 + t95567 + t95569 - F::cast_from(0.14456046980341999104e-1_f64) * t95572 - F::cast_from(0.19274729307122665471e-1_f64) * t95576;
    (t102972, t102977)
}
