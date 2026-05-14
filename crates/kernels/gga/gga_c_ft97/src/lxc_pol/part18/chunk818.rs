//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 818/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk818<F: Float>(t22514: F, t72: F, t35: F, t53: F, t3065: F, t22632: F, t5612: F, t5611: F, t1651: F, t2258: F, t5579: F, t1643: F, t8633: F, t22767: F, t1710: F, t39: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t22797 = t22514 * t72;
    let t22798 = t35 * t53;
    let t22799 = t3065 * t22798;
    let t22800 = t22797 * t22799;
    let t22803 = t22632 * t5612;
    let t22804 = t5611 * t22803;
    let t22806 = t2258 * t1651;
    let t22807 = t5579 * t22806;
    let t22810 = t8633 * t1643;
    let t22811 = t5579 * t22810;
    let t22814 = t22767 * t5612;
    let t22817 = t1710 * t39;
    (t22797, t22798, t22799, t22800, t22803, t22804, t22806, t22807, t22810, t22811, t22814, t22817)
}
