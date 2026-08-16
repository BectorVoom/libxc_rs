//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1370/1444 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1370(t33466: f64, t33468: f64, t33470: f64, t33472: f64, t33477: f64, t33479: f64, t33482: f64, t33487: f64, t33492: f64, t33495: f64, t33501: f64, t33505: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t36599 = 0.70341874126922921074e-8_f64 * t33466;
    let t36600 = 0.15458908518028544927e-5_f64 * t33468;
    let t36601 = 0.2748593934505475288e-5_f64 * t33470;
    let t36602 = 0.22509399720615334744e-6_f64 * t33472;
    let t36604 = 0.9275345110817126956e-4_f64 * t33477;
    let t36605 = 0.9275345110817126956e-4_f64 * t33479;
    let t36606 = 0.77294542590142724634e-6_f64 * t33482;
    let t36609 = 0.1374296967252737644e-5_f64 * t33487;
    let t36610 = 0.24581606547037760418e-7_f64 * t33492;
    let t36611 = 0.13340570901084688392e-7_f64 * t33495;
    let t36612 = 0.26194149710963390811e-9_f64 * t33501;
    let t36613 = 0.24581606547037760418e-8_f64 * t33505;
    (t36599, t36600, t36601, t36602, t36604, t36605, t36606, t36609, t36610, t36611, t36612, t36613)
}
