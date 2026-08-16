//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2299/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2299(t8070: f64, t85660: f64, t225: f64, t27654: f64, t24574: f64, t27484: f64, t1244: f64, t1246: f64, t15018: f64, t15426: f64, t2152: f64, t24589: f64, t24776: f64, t24812: f64, t24820: f64, t24821: f64, t24833: f64, t24849: f64, t27460: f64, t27510: f64, t27532: f64, t3243: f64, t5011: f64, t5075: f64, t7283: f64, t7327: f64, t7348: f64, t7364: f64, t7373: f64, t85883: f64, t85918: f64) -> f64 {
    let t95033 = t85660 * t8070;
    let t95035 = t27654 * t225;
    let t95048 = 0.54831135561607547884e-2_f64 * t24574 * t27484;
    let t95058 = -0.82246703342411321825e-2_f64 * t24812 * t24820 * t15018 * t24821 - 0.27415567780803773942e-2_f64 * t85883 + 0.18277045187202515961e-2_f64 * t95033 + 0.54831135561607547884e-2_f64 * t24589 * t95035 * t7364 - 0.54831135561607547884e-2_f64 * t24849 * t7327 * t5075 * t27532 + 2.0_f64 * t1244 * t7348 * t5011 * t1246 - t95048 + 0.36554090374405031923e-2_f64 * t7283 * t24776 * t27460 * t3243 + t15426 * t2152 - 0.36554090374405031922e-2_f64 * t85918 - 0.16449340668482264365e-1_f64 * t7373 * t24833 * t27510;
    t95058
}
