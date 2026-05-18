//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 664/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk664<F: Float>(t4823: F, t4825: F, t4741: F, t4744: F, t4746: F, t4748: F, t4751: F, t4733: F, t4736: F, t4739: F, t401: F, t384: F) -> (F, F, F, F, F, F, F) {
    let t4826 = t4823 * t4825;
    let t4827 = F::new(0.96491876992155210402e2) * t4826;
    let t4831 = F::new(0.93011851851851851854e0) * t4741;
    let t4832 = F::new(0.13651666666666666667e0) * t4744;
    let t4833 = F::new(0.27303333333333333333e0) * t4746;
    let t4834 = F::new(0.3185388888888888889e0) * t4748;
    let t4835 = F::new(0.36514074074074074075e0) * t4751;
    let t4836 = -F::new(0.25319e1) * t4733 + F::new(0.16879333333333333333e1) * t4736 - F::new(0.19692555555555555555e1) * t4739 - t4831 + t4832 - t4833 - t4834 - t4835;
    let t4837 = t4836 * t401;
    let t4838 = t384 * t4837;
    (t4827, t4831, t4832, t4833, t4834, t4835, t4838)
}
