//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 872/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk872<F: Float>(t22365: F, t2782: F, t22009: F, t4086: F, t543: F, t22005: F, t6888: F, t72: F, t1432: F, t686: F, t213: F, t6918: F, t3915: F, t6889: F, t786: F, t1364: F) -> (F, F, F, F, F, F, F, F) {
    let t22366 = t2782 * t22365;
    let t22369 = t4086 * t22009 * t543;
    let t22370 = t2782 * t22369;
    let t22373 = t4086 * t22005 * t543;
    let t22374 = t2782 * t22373;
    let t22379 = t6888 * t72;
    let t22381 = t1432 * t22379 * t686;
    let t22390 = t213 * t6888;
    let t22398 = t6918 * t72;
    let t22399 = t22398 * t686;
    let t22400 = t3915 * t22399;
    let t22404 = t786 * t6889;
    let t22405 = t22404 * t1364;
    (t22366, t22370, t22374, t22381, t22390, t22399, t22400, t22405)
}
