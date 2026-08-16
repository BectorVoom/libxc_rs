//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1017/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1017<F: Float>(t1081: F, t2410: F, t1020: F, t3386: F, t3648: F, t839: F, t11930: F, t333: F, t335: F, t337: F, t339: F, t341: F) -> (F, F, F, F, F, F, F, F) {
    let t11975 = t2410 * t1081;
    let t11977 = t1020 * t3386;
    let t11979 = t839 * t3648;
    let t11981 = t333 * t11930;
    let t11983 = t335 * t11930;
    let t11985 = t337 * t11930;
    let t11987 = t339 * t11930;
    let t11989 = t341 * t11930;
    (t11975, t11977, t11979, t11981, t11983, t11985, t11987, t11989)
}
