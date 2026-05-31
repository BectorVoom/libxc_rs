//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1605/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1605<F: Float>(t1100: F, t3333: F, t3335: F, t389: F, t2918: F, t936: F, t2874: F, t2926: F, t934: F, t2924: F, t1077: F, t225: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11105 = t3333 * t1100;
    let t11108 = F::cast_from(1.0_f64) / t3335 / t389;
    let t11112 = t936 * t2918;
    let t11114 = F::cast_from(6.0_f64) * t2874 * t11112;
    let t11116 = t2918 * t2926 * t934;
    let t11118 = F::cast_from(0.48245938496077605201e2_f64) * t2924 * t11116;
    let t11119 = t1077 * t1077;
    let t11120 = F::cast_from(1.0_f64) / t11119;
    let t11121 = t225 * t11120;
    (t11105, t11108, t11112, t11114, t11116, t11118, t11119, t11120, t11121)
}
