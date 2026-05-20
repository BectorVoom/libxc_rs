//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2613/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2613<F: Float>(t18352: F, t2710: F, t2713: F, t10722: F, t6030: F, t18419: F, t9775: F, t10777: F, t18481: F, t50945: F, t18333: F, t51123: F) -> (F, F, F, F, F) {
    let t61888 = t2710 * t2713 * t18352;
    let t61890 = t10722 * t6030;
    let t61892 = t9775 * t18419;
    let t61913 = t10777 * t50945 * t18481;
    let t61916 = t10777 * t51123 * t18333;
    (t61888, t61890, t61892, t61913, t61916)
}
