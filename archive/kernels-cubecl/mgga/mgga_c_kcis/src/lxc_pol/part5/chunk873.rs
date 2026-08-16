//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 873/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk873<F: Float>(t1495: F, t7202: F, t1395: F, t1464: F, t2002: F, t5748: F, t3752: F, t3755: F, t6281: F, t1889: F, t1897: F, t3761: F) -> (F, F, F, F, F, F, F) {
    let t7203 = t1495 * t7202;
    let t7204 = t1395 * t7203;
    let t7205 = t1464 * t7204;
    let t7207 = t5748 * t2002;
    let t7208 = t1464 * t7207;
    let t7214 = t3752 * t3755 * t6281;
    let t7218 = t3761 * t1889 * t1897;
    (t7203, t7204, t7205, t7207, t7208, t7214, t7218)
}
