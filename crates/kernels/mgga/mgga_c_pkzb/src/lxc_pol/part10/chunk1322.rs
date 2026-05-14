//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1322/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1322<F: Float>(t1108: F, t17519: F, t17521: F, t1917: F, t1932: F, t1938: F, t1955: F, t1956: F, t1971: F, t1977: F, t21090: F, t21146: F, t21203: F, t21212: F, t25897: F, t25899: F, t25910: F, t25913: F, t25918: F, t3564: F, t3565: F, t3592: F, t3608: F, t5845: F, t5871: F, t7234: F, t7237: F, t7244: F, t7248: F, t7258: F, t7261: F, t7303: F, t7309: F, t7315: F, t7324: F, t7474: F, t7486: F, t7494: F, t9429: F) -> (F,) {
    let t26193 = t25897 - t25899 + t25910 + 0.2069040516770936012e4 * t5871 * t9429 * t1932 + 0.19964560303604640732e6 * t17519 * t3564 * t17521 * t1917 - 0.23392894490538584828e1 * t7494 * t7258 - 0.2077903092681775651e3 * t21146 * t7261 + 0.34631718211362927517e2 * t7315 * t7303 + 0.20508037716432813315e4 * t21212 * t7309 + 0.35089341735807877242e1 * t1977 * t3592 * t1971 + 0.6233709278045326953e3 * t5845 * t3608 * t1956 - 0.23392894490538584828e1 * t1955 * t1108 * t7474 - 4.0 * t7486 * t7234 - 0.38596750796862084161e3 * t21090 * t7237 + 0.64327917994770140268e2 * t7324 * t7244 + 0.4138081033541872024e4 * t21203 * t7248 + 6.0 * t1938 * t3565 * t1932 + t25913 - t25918;
    (t26193,)
}
