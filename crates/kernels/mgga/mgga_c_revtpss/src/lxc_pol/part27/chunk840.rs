//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 840/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk840<F: Float>(t1100: F, t3333: F, t3335: F, t389: F, t2918: F, t936: F, t2874: F, t2926: F, t934: F, t2924: F, t1077: F, t225: F, t1096: F, t3270: F, t1071: F, t3046: F) -> (F, F, F, F, F, F) {
    let t11105 = t3333 * t1100;
    let t11108 = 1.0 / t3335 / t389;
    let t11112 = t936 * t2918;
    let t11114 = 6.0 * t2874 * t11112;
    let t11116 = t2918 * t2926 * t934;
    let t11118 = 0.48245938496077605201e2 * t2924 * t11116;
    let t11119 = t1077 * t1077;
    let t11120 = 1.0 / t11119;
    let t11121 = t225 * t11120;
    let t11122 = t3270 * t1096;
    let t11123 = t11121 * t11122;
    let t11128 = t3046 * t1071;
    (t11105, t11108, t11114, t11118, t11123, t11128)
}
