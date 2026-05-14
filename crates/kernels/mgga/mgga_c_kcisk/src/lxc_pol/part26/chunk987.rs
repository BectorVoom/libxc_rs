//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 987/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk987<F: Float>(t1175: F, t26616: F, t26617: F, t12849: F, t1364: F, t19450: F, t25912: F, t19423: F, t3521: F, t7850: F, t442: F, t7897: F, t1056: F, t3544: F, t7757: F, t13129: F) -> (F, F, F, F, F, F, F) {
    let t26619 = t26616 * t26617 * t1175;
    let t26623 = t12849 * t26617 * t1364;
    let t26626 = t19450 * t25912;
    let t26629 = t19423 * t25912;
    let t26632 = t3521 * t7850;
    let t26634 = t7897 * t442;
    let t26635 = t26634 * t1056;
    let t26636 = t3544 * t26635;
    let t26639 = t7757 * t442;
    let t26641 = t13129 * t26639 * t1056;
    (t26619, t26623, t26626, t26629, t26632, t26636, t26641)
}
