//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2272/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2272(t5837: f64, t984: f64, t23384: f64, t28691: f64, t28705: f64, t82431: f64, t14545: f64, t1635: f64, t18070: f64, t1956: f64, t23327: f64, t23336: f64, t23372: f64, t25420: f64, t25429: f64, t25750: f64, t25797: f64, t28491: f64, t4557: f64, t5944: f64, t61646: f64, t6687: f64, t6704: f64, t7565: f64, t7600: f64, t82481: f64, t88162: f64, t88167: f64, t88194: f64, t88744: f64, t89598: f64) -> (f64, f64) {
    let t99180 = t5837 * t984;
    let t99184 = t23384 * t28691;
    let t99190 = t82431 * t28705;
    let t99202 = -0.16449340668482264365e-1_f64 * t6687 * t89598 * t7565 + 4.0_f64 * t4557 * t25420 + 4.0_f64 * t14545 * t7600 - t88167 - 0.82246703342411321825e-2_f64 * t6687 * t99180 * t25797 - 0.27415567780803773942e-2_f64 * t99184 - t23372 * t5944 - 0.36554090374405031923e-2_f64 * t25429 * t23336 * t28491 - 0.18277045187202515961e-2_f64 * t99190 - 2.0_f64 * t88744 * t1635 - 0.49348022005446793095e-1_f64 * t6687 * t6704 * t82481 * t18070 - t61646 * t1956 + t88194 + 0.54831135561607547883e-2_f64 * t23327 * t88162 * t25750;
    (t99180, t99202)
}
