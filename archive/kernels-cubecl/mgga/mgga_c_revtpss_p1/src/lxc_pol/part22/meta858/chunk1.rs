//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3006/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3006<F: Float>(t10722: F, t4345: F, t40710: F, t4349: F, t14834: F, t9775: F, t10716: F, t14857: F, t2475: F, t4343: F, t14832: F, t2661: F, t775: F) -> (F, F, F, F, F) {
    let t50383 = t10722 * t4345;
    let t50385 = t40710 * t4349;
    let t50387 = t9775 * t14834;
    let t50389 = t10716 * t14857;
    let t50391 = t2475 * t4343;
    let t50394 = t2661 * t14832 * t50391 * t775;
    (t50383, t50385, t50387, t50389, t50394)
}
