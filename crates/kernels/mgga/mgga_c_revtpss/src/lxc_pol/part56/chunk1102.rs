//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1102/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1102<F: Float>(t121333: F, t125693: F, t121240: F, t120975: F, t1885: F, t121058: F, t121074: F, t121087: F, t121091: F, t121094: F, t121098: F, t121101: F, t121108: F, t121111: F, t121117: F, t121135: F, t125677: F, t125681: F, t125691: F, t14230: F, t32700: F, t32719: F, t33956: F, t7274: F, t7910: F, t8578: F, t8706: F, t8707: F, t9990: F) -> F {
    let t125694 = t121333 * t125693;
    let t125696 = t121240 * t125693;
    let t125706 = t120975 * t1885;
    let t125712 = F::new(0.112937867033921868e-2) * t125677 + F::new(0.131760844872908846e-2) * t121058 - t121074 + F::new(0.25389723392137995738e-1) * t125681 - F::new(0.28559868832551176308e-1) * t121087 + F::new(0.28912093960683998208e-1) * t121091 + F::new(0.34271842599061411569e1) * t32719 * t9990 * t8578 * t14230 - F::new(0.76169170176413987216e-1) * t125691 + F::new(0.28559868832551176308e-1) * t125694 - F::new(0.50779446784275991476e-1) * t125696 + F::new(0.11423947533020470523e1) * t8706 * t8707 * t7274 * t7910 - F::new(0.51405703062096148812e-1) * t121094 + t121098 + F::new(0.11423947533020470523e1) * t32700 * t33956 - F::new(0.33059535666846348619e-4) * t121101 - F::new(0.17354086964223805049e-2) * t125706 - F::new(0.3718732920905101082e-4) * t121108 + F::new(0.66119071333692697238e-4) * t121111 + F::new(0.131760844872908846e-2) * t121117 + F::new(0.42839803248826764462e-1) * t121135;
    t125712
}
