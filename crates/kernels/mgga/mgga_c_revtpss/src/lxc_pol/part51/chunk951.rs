//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 951/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk951<F: Float>(t121309: F, t786: F, t122: F, t32219: F, t3916: F, t119833: F, t121245: F, t121248: F, t121239: F, t25875: F, t121241: F, t121116: F, t32208: F, t7063: F, t121305: F, t32186: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t121310 = t786 * t121309;
    let t121312 = t32219 * t122 * t3916;
    let t121313 = t121310 * t121312;
    let t121326 = t119833 * t121245;
    let t121327 = t121326 * t121248;
    let t121333 = t25875 * t121239;
    let t121334 = t121333 * t121241;
    let t121336 = t121116 * t32208;
    let t121338 = t7063 * t121309;
    let t121339 = t121338 * t121312;
    let t121342 = t7063 * t121305 * t32186;
    (t121310, t121313, t121326, t121327, t121333, t121334, t121336, t121338, t121339, t121342)
}
