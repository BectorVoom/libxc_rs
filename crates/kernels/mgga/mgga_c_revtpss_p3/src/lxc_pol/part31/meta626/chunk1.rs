//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2079/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2079<F: Float>(t1032: F, t4930: F, t994: F, t15669: F, t1976: F, t1035: F, t1983: F, t99682: F, t25698: F, t93920: F, t1647: F, t7135: F) -> (F, F, F, F, F, F) {
    let t99708 = t4930 * t1032;
    let t99709 = t994 * t99708;
    let t99721 = t15669 * t1976;
    let t99743 = t1983 * t99682 * t1035;
    let t99824 = t25698 * t93920;
    let t99881 = t1647 * t7135;
    (t99708, t99709, t99721, t99743, t99824, t99881)
}
