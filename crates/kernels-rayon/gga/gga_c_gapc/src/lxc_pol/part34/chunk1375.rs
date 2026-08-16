//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1375/1427 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1375(t33492: f64, t33495: f64, t33501: f64, t33505: f64, t33510: f64, t33513: f64, t33518: f64, t33528: f64, t33532: f64, t33507: f64, t36609: f64, t33536: f64) -> (f64, f64) {
    let t36610 = 0.24581606547037760418e-7_f64 * t33492;
    let t36611 = 0.13340570901084688392e-7_f64 * t33495;
    let t36612 = 0.26194149710963390811e-9_f64 * t33501;
    let t36613 = 0.24581606547037760418e-8_f64 * t33505;
    let t36615 = 0.63350674672043801542e-5_f64 * t33510;
    let t36616 = 0.49520679385353736436e-5_f64 * t33513;
    let t36617 = 0.96681162811134562538e-8_f64 * t33518;
    let t36618 = 0.28198672486580914074e-8_f64 * t33528;
    let t36619 = 0.57920616843011475696e-5_f64 * t33532;
    let t36620 = -t36609 - t36610 + t36611 - t36612 + t36613 + 0.67632724766374884053e-5_f64 * t33507 - t36615 - t36616 + t36617 + t36618 - t36619;
    let t36621 = 0.44197102999375800017e-7_f64 * t33536;
    (t36620, t36621)
}
