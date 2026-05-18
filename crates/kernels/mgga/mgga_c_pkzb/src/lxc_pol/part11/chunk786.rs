//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 786/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk786<F: Float>(t1123: F, t5728: F, t2099: F, t2918: F, t757: F, t2946: F, t300: F, t779: F) -> (F, F, F, F, F) {
    let t7665 = t1123 * t5728;
    let t7692 = t2099 * t2918;
    let t7694 = F::new(0.28582678745379824648e-3) * t757 * t7692;
    let t7695 = t300 * t2946;
    let t7699 = t779 * t1123;
    let t7700 = t300 * t7699;
    (t7665, t7694, t7695, t7699, t7700)
}
