//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 991/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk991<F: Float>(t1417: F, t7879: F, t7874: F, t7899: F, t425: F, t7897: F, t1175: F, t3564: F, t1364: F, t8111: F, t5953: F, t1056: F, t26617: F, t19412: F, t19419: F, t12849: F, t25911: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t26710 = t1417 * t7879;
    let t26712 = t1417 * t7874;
    let t26714 = t1417 * t7899;
    let t26717 = t425 * t7897;
    let t26718 = t26717 * t1175;
    let t26719 = t3564 * t26718;
    let t26722 = t8111 * t1364;
    let t26723 = t5953 * t26722;
    let t26726 = t26617 * t1056;
    let t26727 = t19412 * t26726;
    let t26730 = t19419 * t26726;
    let t26734 = t12849 * t25911 * t1175;
    (t26710, t26712, t26714, t26718, t26719, t26722, t26723, t26727, t26730, t26734)
}
