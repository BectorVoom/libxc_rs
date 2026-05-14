//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 549/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk549<F: Float>(t255: F, t9802: F, t1882: F, t2471: F, t731: F, t8232: F, t768: F, t2563: F, t2559: F, t9723: F, t9727: F, t9730: F, t9520: F, t9768: F, t9765: F, t251: F, t631: F, t675: F, t7242: F, t898: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t9803 = t9802 * t255;
    let t9813 = t1882 * t2471;
    let t9822 = t8232 * t731;
    let t9824 = t8232 * t768;
    let t9826 = t1882 * t2563;
    let t9828 = t1882 * t2559;
    let t9861 = t9723 / 9.0;
    let t9862 = 2.0 / 27.0 * t9727;
    let t9869 = 2.0 / 3.0 * t9730;
    let t9870 = t9520 / 3.0;
    let t9872 = 2.0 / 9.0 * t9768;
    let t9876 = 2.0 / 9.0 * t9765;
    let t9890 = 1.0 / t251 / t631 / t898 / t675 / t7242 / 4.0;
    (t9803, t9813, t9822, t9824, t9826, t9828, t9861, t9862, t9869, t9870, t9872, t9876, t9890)
}
