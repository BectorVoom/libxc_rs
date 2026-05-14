//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 844/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk844<F: Float>(t28108: F, t77: F, t1470: F, t2242: F, t1923: F, t1928: F, t25106: F, t28078: F, t28081: F, t28086: F, t28090: F, t28093: F, t28105: F, t6954: F, t6958: F, t6974: F, t6978: F, t7702: F, t7706: F, t7716: F, t7720: F) -> (F, F, F) {
    let t28109 = t77 * t28108;
    let t28112 = t2242 * t1470;
    let t28115 = -t1923 * t28078 / 6.0 - t1923 * t28081 / 6.0 - t6954 * t7720 / 6.0 - t1923 * t28086 / 6.0 - t1923 * t28090 / 6.0 - t28093 * t1928 / 6.0 - t7702 * t6974 / 6.0 - t7702 * t6978 / 6.0 - t6954 * t7716 / 6.0 + 5.0 / 6.0 * t25106 * t7706 + 5.0 / 6.0 * t6958 * t28105 + 5.0 / 6.0 * t6958 * t28109 + t28112 * t1928 / 3.0;
    (t28109, t28112, t28115)
}
