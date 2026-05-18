//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1290/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1290<F: Float>(t1983: F, t93982: F, t1096: F, t4982: F, t1043: F, t1976: F, t3153: F, t1981: F, t42058: F, t7143: F, t1982: F, t93484: F) -> (F, F, F, F, F) {
    let t93983 = t1983 * t93982;
    let t93984 = t4982 * t1096;
    let t93988 = t1976 * t1043;
    let t93989 = t93988 * t3153;
    let t93994 = t1981 * t42058 * t7143;
    let t94005 = t1982 * t93484;
    (t93983, t93984, t93989, t93994, t94005)
}
