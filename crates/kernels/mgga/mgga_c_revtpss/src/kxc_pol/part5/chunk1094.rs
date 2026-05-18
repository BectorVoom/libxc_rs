//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1094/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1094<F: Float>(t15127: F, t4625: F, t698: F, t4622: F, t1593: F, t2435: F) -> (F, F, F, F, F) {
    let t15128 = F::new(0.13418888888888888889e0) * t15127;
    let t15168 = t698 * t4625;
    let t15169 = F::new(0.22076e0) * t15168;
    let t15170 = t698 * t4622;
    let t15189 = t2435 * t1593;
    (t15128, t15168, t15169, t15170, t15189)
}
