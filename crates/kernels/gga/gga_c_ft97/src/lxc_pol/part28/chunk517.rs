//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 517/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk517<F: Float>(t22514: F, t72: F, t35: F, t53: F, t22632: F, t5612: F, t5611: F, t1710: F, t39: F) -> (F, F, F, F, F) {
    let t22797 = t22514 * t72;
    let t22798 = t35 * t53;
    let t22803 = t22632 * t5612;
    let t22804 = t5611 * t22803;
    let t22817 = t1710 * t39;
    (t22797, t22798, t22803, t22804, t22817)
}
