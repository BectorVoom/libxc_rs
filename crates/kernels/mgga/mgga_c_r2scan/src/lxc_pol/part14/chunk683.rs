//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 683/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk683<F: Float>(t1664: F, t5767: F, t390: F, t1800: F, t189: F, t1658: F, t5448: F, t649: F, t1957: F, t206: F, t5632: F, t664: F, t2006: F, t2008: F, t1966: F, t188: F, t650: F) -> (F, F, F, F, F, F, F, F, F) {
    let t5768 = t5767 * t1664;
    let t5770 = 0.17183595094352973719e1 * t390 * t5768;
    let t5771 = t189 * t1800;
    let t5772 = t1658 * t5771;
    let t5774 = 0.10685e0 * t390 * t5772;
    let t5777 = 0.12822e1 * t649 * t5448 * t189;
    let t5781 = t1957 * t206;
    let t5782 = t5632 * t664;
    let t5785 = t2006 * t206;
    let t5786 = t2008 * t664;
    let t5787 = t5786 * t1966;
    let t5790 = t650 * t188;
    (t5770, t5771, t5774, t5777, t5781, t5782, t5785, t5787, t5790)
}
