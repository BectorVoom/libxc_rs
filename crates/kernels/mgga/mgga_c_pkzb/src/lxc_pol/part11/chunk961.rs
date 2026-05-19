//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 961/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk961<F: Float>(t12: F, t10513: F, t10518: F, t2540: F, t3366: F, t5093: F, t87: F, t1003: F, t3371: F, zeta_threshold: F) -> (F, F) {
    let t84 = t12 <= zeta_threshold;
    let t10522 = piecewise3::<F>(t84, F::new(0.0), -F::new(8.0) / F::new(27.0) * t5093 * t10513 + F::new(4.0) / F::new(3.0) * t2540 * t3366 + F::new(4.0) / F::new(3.0) * t87 * t10518);
    let t10523 = t3371 * t1003;
    (t10522, t10523)
}
