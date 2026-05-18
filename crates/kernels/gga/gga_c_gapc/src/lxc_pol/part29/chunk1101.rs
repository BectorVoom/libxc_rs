//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 1101/1311 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk1101<F: Float>(t21801: F, t2660: F, t7330: F, t1086: F, t11986: F, t22783: F, t11311: F, t11987: F, t8117: F, t11483: F, t15843: F, t2597: F, t2675: F) -> (F, F, F, F) {
    let t33631 = t2660 * t21801 * t7330;
    let t33634 = t11986 * t1086 * t22783;
    let t33637 = t8117 * t11311 * t11987;
    let t33641 = t2675 * t11483 * t2597 * t15843;
    (t33631, t33634, t33637, t33641)
}
