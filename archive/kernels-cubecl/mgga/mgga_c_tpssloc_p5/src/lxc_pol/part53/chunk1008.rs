//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 1008/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk1008<F: Float>(t22892: F, t22893: F, t33276: F, t22751: F, t33277: F, t552: F, t7918: F, t1307: F, t6637: F, t6888: F, t33245: F, t6897: F, t794: F) -> (F, F, F, F) {
    let t122533 = t22892 * t22893 * t33276;
    let t122535 = t22751 * t33277;
    let t122537 = t552 * t7918;
    let t122540 = t6888 * t6637 * t122537 * t1307;
    let t122551 = t6897 * t794 * t33245;
    (t122533, t122535, t122540, t122551)
}
