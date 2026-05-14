//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 231/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk231<F: Float>(t1043: F, t373: F, t357: F, t73: F, t1042: F, t362: F, t39: F, t40: F, t361: F, t351: F, t127: F, t371: F, t367: F, t365: F, t369: F) -> (F, F, F, F, F, F, F, F, F) {
    let t1044 = t373 * t1043;
    let t1045 = t73 * t357;
    let t1046 = t1044 * t1045;
    let t1047 = t1042 * t1046;
    let t1050 = t362 * t39;
    let t1052 = 1.0 / t40 / t1050;
    let t1053 = t361 * t1052;
    let t1054 = t351 * t1053;
    let t1058 = t371 * t127 * t373;
    let t1060 = 0.14291339372689912324e-3 * t367 * t1058;
    let t1061 = t365 * t369;
    let t1062 = t361 * t1061;
    (t1045, t1046, t1047, t1052, t1053, t1054, t1058, t1060, t1062)
}
