//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 935/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk935<F: Float>(t121248: F, t121326: F, t121239: F, t25875: F, t121241: F, t121116: F, t32208: F, t121309: F, t7063: F, t121312: F, t121305: F, t32186: F, t119900: F, t121165: F, t240: F, t545: F) -> (F, F, F, F, F, F, F, F) {
    let t121327 = t121326 * t121248;
    let t121333 = t25875 * t121239;
    let t121334 = t121333 * t121241;
    let t121336 = t121116 * t32208;
    let t121338 = t7063 * t121309;
    let t121339 = t121338 * t121312;
    let t121342 = t7063 * t121305 * t32186;
    let t121343 = 0.35698404904233436678e-3 * t121342;
    let t121346 = t119900 * t545 * t240 * t121165;
    (t121327, t121333, t121334, t121336, t121338, t121339, t121343, t121346)
}
