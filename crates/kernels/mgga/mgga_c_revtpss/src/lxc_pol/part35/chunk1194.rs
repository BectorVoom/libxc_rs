//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1194/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1194<F: Float>(t110110: F, t111221: F, t114373: F, t1450: F, t1519: F, t18245: F, t2014: F, t2055: F, t2056: F, t2106: F, t2107: F, t22475: F, t22813: F, t22852: F, t25043: F, t25045: F, t28167: F, t28653: F, t28938: F, t29494: F, t30209: F, t30218: F, t30563: F, t30571: F, t30581: F, t4248: F, t5887: F, t5921: F, t651: F, t7359: F, t75941: F, t7732: F, t7898: F, t7900: F, t7978: F, t8108: F, t86791: F, t9069: F) -> F {
    let t114986 = F::new(6.0) * t2014 * t8108 * t22475 + F::new(9.0) * t7898 * t30581 - F::new(6.0) * t7359 * t25045 + F::new(9.0) * t2014 * t28938 * t29494 - F::new(2.0) * t75941 * t2056 - F::new(6.0) * t114373 * t2056 - F::new(6.0) * t18245 * t7978 - F::new(6.0) * t7898 * t30218 - F::new(6.0) * t110110 * t1519 - F::new(12.0) * t28653 * t5887 - F::new(6.0) * t7732 * t30571 - F::new(6.0) * t4248 * t30563 - F::new(6.0) * t7732 * t30563 - F::new(2.0) * t651 * t25043 * t2055 - F::new(12.0) * t7732 * t30209 - F::new(6.0) * t28653 * t5921 + F::new(18.0) * t28167 * t9069 * t22852 - F::new(6.0) * t2014 * t2107 * t86791 + F::new(6.0) * t2014 * t22813 * t2106 * t1450 + F::new(9.0) * t2014 * t111221 * t7900;
    t114986
}
