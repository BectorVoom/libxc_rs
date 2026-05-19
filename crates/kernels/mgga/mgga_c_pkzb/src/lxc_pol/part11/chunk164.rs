//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 164/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk164<F: Float>(t12: F, t127: F, t501: F, t439: F, t87: F, zeta_threshold: F) -> (F, F, F) {
    let t84 = t12 <= zeta_threshold;
    let t503 = F::new(4.0) * t501 * t127;
    let t506 = piecewise3::<F>(t84, F::new(0.0), F::new(4.0) / F::new(3.0) * t87 * t439);
    let t507 = -t439;
    (t503, t506, t507)
}
