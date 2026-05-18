//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1377/1429 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1377<F: Float>(t33492: F, t33495: F, t33501: F, t33505: F, t33510: F, t33513: F, t33518: F, t33528: F, t33532: F, t33507: F, t36609: F, t33536: F) -> (F, F) {
    let t36610 = F::new(0.24581606547037760418e-7) * t33492;
    let t36611 = F::new(0.13340570901084688392e-7) * t33495;
    let t36612 = F::new(0.26194149710963390811e-9) * t33501;
    let t36613 = F::new(0.24581606547037760418e-8) * t33505;
    let t36615 = F::new(0.63350674672043801542e-5) * t33510;
    let t36616 = F::new(0.49520679385353736436e-5) * t33513;
    let t36617 = F::new(0.96681162811134562538e-8) * t33518;
    let t36618 = F::new(0.28198672486580914074e-8) * t33528;
    let t36619 = F::new(0.57920616843011475696e-5) * t33532;
    let t36620 = -t36609 - t36610 + t36611 - t36612 + t36613 + F::new(0.67632724766374884053e-5) * t33507 - t36615 - t36616 + t36617 + t36618 - t36619;
    let t36621 = F::new(0.44197102999375800017e-7) * t33536;
    (t36620, t36621)
}
