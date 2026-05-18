//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 904/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk904<F: Float>(t1724: F, t2943: F, t4667: F, t932: F, t13744: F, t1670: F, t9758: F, t2944: F, t4625: F, t934: F, t2952: F, t4657: F) -> (F, F, F, F, F, F) {
    let t13747 = t2943 * t1724;
    let t13750 = t932 * t4667;
    let t13767 = t932 * t13744;
    let t13771 = t9758 * t1670;
    let t13772 = t13771 * t2944;
    let t13774 = t2943 * t4625;
    let t13775 = t13774 * t934;
    let t13777 = t4657 * t2952;
    (t13747, t13750, t13767, t13772, t13775, t13777)
}
