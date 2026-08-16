//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 953/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk953(t112778: f64, t112803: f64, t112818: f64, t112820: f64, t112773: f64, t112782: f64, t112784: f64, t112788: f64, t112795: f64, t112798: f64, t112807: f64, t112811: f64, t112814: f64) -> f64 {
    let t114714 = 0.5383034145885385447e-3_f64 * t112778;
    let t114720 = 7.0_f64 / 576.0_f64 * t112803;
    let t114724 = 0.32298204875312312682e-2_f64 * t112818;
    let t114725 = 7.0_f64 / 144.0_f64 * t112820;
    let t114726 = t112773 / 96.0_f64 + t114714 + 0.67826230238155856632e-1_f64 * t112782 + 0.13565246047631171327e0_f64 * t112784 - 0.96894614625936938046e-2_f64 * t112788 + t112795 / 384.0_f64 - t112798 / 384.0_f64 + t114720 - t112807 / 768.0_f64 - t112811 / 768.0_f64 + 0.32298204875312312682e-2_f64 * t112814 + t114724 + t114725;
    t114726
}
