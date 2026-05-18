//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1110/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1110<F: Float>(t2035: F, t25188: F, t531: F, t7311: F, t7238: F, t2014: F, t7312: F, t7315: F, t1310: F, t1453: F, t1932: F, t2007: F, t2320: F, t2328: F, t25078: F, t25085: F, t25092: F, t25095: F, t25096: F, t25169: F, t25180: F, t25182: F, t25184: F, t25186: F, t3813: F, t508: F, t649: F, t651: F, t6983: F, t7221: F, t7231: F) -> (F, F, F, F) {
    let t25189 = t25188 * t2035;
    let t25190 = t531 * t7311;
    let t25191 = t25190 * t7238;
    let t25193 = F::new(6.0) * t2014 * t25191;
    let t25194 = t7312 * t7315;
    let t25196 = F::new(2.0) * t2014 * t25194;
    let t25197 = -F::new(2.0) * t1310 * t6983 + F::new(2.0) * t1453 * t7231 - t1932 * t3813 - t2007 * t2320 - F::new(2.0) * t2007 * t2328 - F::new(2.0) * t25078 * t651 - F::new(2.0) * t25096 * t508 - t25169 * t508 - F::new(2.0) * t649 * t7221 - t25085 + t25092 - t25095 + t25180 - t25182 - t25184 - t25186 + t25189 + t25193 - t25196;
    (t25190, t25191, t25194, t25197)
}
