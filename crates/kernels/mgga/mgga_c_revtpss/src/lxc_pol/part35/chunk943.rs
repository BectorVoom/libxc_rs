//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 943/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk943<F: Float>(t545: F, t9656: F, t4075: F, t7282: F, t1955: F, t1426: F, t2453: F, t7283: F, t3974: F, t7259: F, t2482: F, t27: F, t7269: F, t2019: F, t3985: F, t820: F, t843: F) -> (F, F, F, F, F, F, F, F, F) {
    let t25924 = t9656 * t545;
    let t25929 = t7282 * t4075;
    let t25930 = t1955 * t25929;
    let t25937 = t1426 * t545;
    let t25944 = t2453 * t7283;
    let t25969 = t7259 * t3974;
    let t25972 = t2482 * t7269 * t27;
    let t25975 = t2019 * t3985;
    let t25978 = t820 * t7269 * t843;
    (t25924, t25929, t25930, t25937, t25944, t25969, t25972, t25975, t25978)
}
