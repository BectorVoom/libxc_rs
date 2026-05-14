//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 970/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk970<F: Float>(t2014: F, t33651: F, t7312: F, t119578: F, t27154: F, t28167: F, t37956: F, t5627: F, t33602: F, t7003: F, t25805: F, t7735: F, t28025: F, t27137: F, t6985: F, t2322: F, t33574: F) -> (F, F, F, F, F, F, F, F) {
    let t125531 = 2.0 * t2014 * t7312 * t33651;
    let t125532 = t119578 * t27154;
    let t125536 = 6.0 * t28167 * t37956 * t5627;
    let t125537 = t33602 * t7003;
    let t125539 = t25805 * t7735;
    let t125541 = t28025 * t7735;
    let t125543 = t6985 * t27137;
    let t125545 = t2322 * t33574;
    (t125531, t125532, t125536, t125537, t125539, t125541, t125543, t125545)
}
