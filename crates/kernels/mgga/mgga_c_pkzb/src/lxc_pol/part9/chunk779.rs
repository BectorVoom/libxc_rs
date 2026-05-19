//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 779/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk779<F: Float>(t12: F, t1835: F, t87: F, t1837: F, t439: F, t1646: F, t5094: F, t5100: F, t652: F, zeta_threshold: F) -> (F, F) {
    let t84 = t12 <= zeta_threshold;
    let t5528 = F::new(1.0) / t87 / t1835 / t12;
    let t5531 = t1837 * t439;
    let t5537 = piecewise3::<F>(t84, F::new(0.0), -F::new(28.0) / F::new(27.0) * t5528 * t5094 + F::new(4.0) / F::new(3.0) * t5531 * t1646 - t652 * t5100 / F::new(3.0));
    (t5528, t5537)
}
