//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 527/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk527<F: Float>(t12: F, t652: F, t8: F, t1429: F, t2732: F, t439: F, zeta_threshold: F) -> (F, F) {
    let t84 = t12 <= zeta_threshold;
    let t2735 = t652 * t8;
    let t2739 = piecewise3::<F>(t84, F::new(0.0), F::new(4.0) / F::new(9.0) * t2732 * t439 - F::new(2.0) / F::new(3.0) * t2735 * t1429);
    (t2735, t2739)
}
