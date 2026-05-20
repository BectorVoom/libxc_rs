//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1364/1505 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1364<F: Float>(t17661: F, t5401: F, t1214: F, t1715: F, t1250: F, t17353: F, t5052: F, t5406: F, t1794: F, t3617: F, t372: F, t5047: F) -> (F, F, F, F, F, F) {
    let t20929 = t17661 * t5401;
    let t20932 = t1715 * t1214;
    let t20933 = t1250 * t20932;
    let t20934 = t17353 * t20933;
    let t20937 = t1250 * t5052;
    let t20938 = t17353 * t20937;
    let t20941 = t17661 * t5406;
    let t20944 = t3617 * t1794;
    let t20945 = t372 * t20944;
    let t20946 = t1250 * t5047;
    (t20929, t20934, t20938, t20941, t20945, t20946)
}
