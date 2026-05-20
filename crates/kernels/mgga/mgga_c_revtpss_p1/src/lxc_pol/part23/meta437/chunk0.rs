//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1847/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1847<F: Float>(t11354: F, t6113: F, t918: F, t4598: F, t4606: F, t2880: F, t6120: F, t11358: F, t4614: F, t2897: F, t18950: F, t916: F) -> (F, F, F, F, F, F, F, F, F) {
    let t18979 = t11354 * t6113;
    let t18980 = t18979 * t918;
    let t18982 = t4598 * t4606;
    let t18984 = t2880 * t6120;
    let t18985 = t18984 * t918;
    let t18987 = t11358 * t6113;
    let t18988 = t18987 * t918;
    let t18990 = t4614 * t4606;
    let t18992 = t2897 * t6120;
    let t18993 = t18992 * t918;
    let t18995 = t916 * t18950;
    (t18979, t18980, t18982, t18985, t18987, t18988, t18990, t18993, t18995)
}
