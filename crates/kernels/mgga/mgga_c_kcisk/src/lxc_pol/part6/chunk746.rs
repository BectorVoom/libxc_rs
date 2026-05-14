//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 746/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk746<F: Float>(t3521: F, t7850: F, t7854: F, t1417: F, t7879: F, t7874: F, t7899: F, t7866: F, t3739: F, t7908: F, t3748: F, t8181: F, t7839: F, t7833: F, t13959: F, t8172: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t26632 = t3521 * t7850;
    let t26692 = t3521 * t7854;
    let t26710 = t1417 * t7879;
    let t26712 = t1417 * t7874;
    let t26714 = t1417 * t7899;
    let t26746 = t1417 * t7866;
    let t26755 = t3739 * t7908;
    let t26764 = t3748 * t8181;
    let t26785 = t3739 * t7839;
    let t26787 = t3739 * t7833;
    let t26841 = t13959 * t8172;
    (t26632, t26692, t26710, t26712, t26714, t26746, t26755, t26764, t26785, t26787, t26841)
}
