//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 916/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk916<F: Float>(t18953: F, t5625: F, t3484: F, t3482: F, t13406: F, t2152: F, t1340: F, t1411: F, t1404: F, t3529: F, t1336: F, t140: F, t5636: F, t2262: F, t3583: F, t3796: F) -> (F, F, F, F, F, F, F) {
    let t18954 = t5625 * t18953;
    let t18955 = t3484 * t18954;
    let t18956 = t3482 * t18955;
    let t18958 = t13406 * t2152;
    let t18959 = t1340 * t18958;
    let t18960 = t1411 * t18959;
    let t18962 = t3529 * t1404;
    let t18964 = t140 * t1336 * t18962;
    let t18965 = t18964 * t5636;
    let t18967 = t2262 * t3583;
    let t18968 = t3796 * t18967;
    (t18954, t18956, t18958, t18960, t18965, t18967, t18968)
}
