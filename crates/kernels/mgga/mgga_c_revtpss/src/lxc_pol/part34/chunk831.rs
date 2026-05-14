//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 831/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk831<F: Float>(t127: F, t371: F, t6276: F, t1025: F, t4845: F, t4858: F, t3172: F, t6307: F, t3150: F, t4820: F, t4879: F, t11725: F, t247: F, t6092: F, t1063: F, t3109: F, t6100: F) -> (F, F, F, F, F, F, F, F, F) {
    let t20020 = t371 * t127 * t6276;
    let t20021 = t1025 * t20020;
    let t20025 = t4858 * t4845;
    let t20029 = t3172 * t6307;
    let t20030 = t3150 * t20029;
    let t20034 = t4879 * t4820;
    let t20050 = t247 * t11725 * t6092;
    let t20051 = t1063 * t20050;
    let t20054 = t247 * t3109 * t6100;
    (t20020, t20021, t20025, t20029, t20030, t20034, t20050, t20051, t20054)
}
