//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1373/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1373<F: Float>(t33466: F, t33468: F, t33470: F, t33472: F, t33477: F, t33479: F, t33482: F, t33487: F, t33492: F, t33495: F, t33501: F, t33505: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t36599 = F::new(0.70341874126922921074e-8) * t33466;
    let t36600 = F::new(0.15458908518028544927e-5) * t33468;
    let t36601 = F::new(0.2748593934505475288e-5) * t33470;
    let t36602 = F::new(0.22509399720615334744e-6) * t33472;
    let t36604 = F::new(0.9275345110817126956e-4) * t33477;
    let t36605 = F::new(0.9275345110817126956e-4) * t33479;
    let t36606 = F::new(0.77294542590142724634e-6) * t33482;
    let t36609 = F::new(0.1374296967252737644e-5) * t33487;
    let t36610 = F::new(0.24581606547037760418e-7) * t33492;
    let t36611 = F::new(0.13340570901084688392e-7) * t33495;
    let t36612 = F::new(0.26194149710963390811e-9) * t33501;
    let t36613 = F::new(0.24581606547037760418e-8) * t33505;
    (t36599, t36600, t36601, t36602, t36604, t36605, t36606, t36609, t36610, t36611, t36612, t36613)
}
