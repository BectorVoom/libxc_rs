//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1194/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1194(t110110: f64, t111221: f64, t114373: f64, t1450: f64, t1519: f64, t18245: f64, t2014: f64, t2055: f64, t2056: f64, t2106: f64, t2107: f64, t22475: f64, t22813: f64, t22852: f64, t25043: f64, t25045: f64, t28167: f64, t28653: f64, t28938: f64, t29494: f64, t30209: f64, t30218: f64, t30563: f64, t30571: f64, t30581: f64, t4248: f64, t5887: f64, t5921: f64, t651: f64, t7359: f64, t75941: f64, t7732: f64, t7898: f64, t7900: f64, t7978: f64, t8108: f64, t86791: f64, t9069: f64) -> f64 {
    let t114986 = 6.0_f64 * t2014 * t8108 * t22475 + 9.0_f64 * t7898 * t30581 - 6.0_f64 * t7359 * t25045 + 9.0_f64 * t2014 * t28938 * t29494 - 2.0_f64 * t75941 * t2056 - 6.0_f64 * t114373 * t2056 - 6.0_f64 * t18245 * t7978 - 6.0_f64 * t7898 * t30218 - 6.0_f64 * t110110 * t1519 - 12.0_f64 * t28653 * t5887 - 6.0_f64 * t7732 * t30571 - 6.0_f64 * t4248 * t30563 - 6.0_f64 * t7732 * t30563 - 2.0_f64 * t651 * t25043 * t2055 - 12.0_f64 * t7732 * t30209 - 6.0_f64 * t28653 * t5921 + 18.0_f64 * t28167 * t9069 * t22852 - 6.0_f64 * t2014 * t2107 * t86791 + 6.0_f64 * t2014 * t22813 * t2106 * t1450 + 9.0_f64 * t2014 * t111221 * t7900;
    t114986
}
