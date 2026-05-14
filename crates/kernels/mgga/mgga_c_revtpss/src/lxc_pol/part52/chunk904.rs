//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 904/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk904<F: Float>(t28699: F, t28729: F, t28759: F, t28942: F, t3: F, t2055: F, t670: F, t1518: F, t26733: F, t4292: F, t7553: F, t116: F, t7983: F, t117: F, t28683: F, t1459: F, t1461: F, t1916: F, t1918: F, t2113: F, t2115: F, t572: F, t573: F, t5795: F, t5802: F, t5805: F, t7547: F, t7554: F, t7557: F, t8118: F, t8124: F, t8127: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t28944 = t28699 + t28729 + t28759 + t28942;
    let t28945 = t3 * t28944;
    let t28956 = param_d * t28944;
    let t28974 = t670 * t2055;
    let t28975 = t28974 * t1518;
    let t28978 = t26733 * t1518;
    let t28981 = t7553 * t4292;
    let t28986 = t116 * t7983;
    let t28987 = t28986 * t670;
    let t28990 = t117 * t28683;
    let t28993 = 6.0 * t1459 * t8124 + 3.0 * t1459 * t8127 + 3.0 * t1461 * t8118 + 6.0 * t1916 * t7554 + 3.0 * t1916 * t7557 + 3.0 * t1918 * t7547 + 6.0 * t2113 * t5802 + 3.0 * t2113 * t5805 + 3.0 * t2115 * t5795 + t28956 * t573 + 6.0 * t28975 * t572 + 6.0 * t28978 * t572 + 6.0 * t28981 * t572 + 6.0 * t28987 * t572 + 3.0 * t28990 * t572;
    (t28945, t28956, t28974, t28975, t28978, t28981, t28986, t28987, t28990, t28993)
}
