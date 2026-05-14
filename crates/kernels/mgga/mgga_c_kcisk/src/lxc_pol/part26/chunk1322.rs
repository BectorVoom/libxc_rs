//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1322/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1322<F: Float>(t34739: F, t3748: F, t1286: F, t1411: F, t8011: F, t9461: F, t20160: F, t34743: F, t9426: F, t1339: F, t32000: F, t34815: F, t26508: F, t34714: F, t3973: F, t9446: F) -> (F, F, F, F, F, F, F) {
    let t119124 = t3748 * t34739;
    let t119128 = t1411 * t9461 * t8011 * t1286;
    let t119130 = t20160 * t34743;
    let t119131 = t9426 * t119130;
    let t119141 = t1339 * t32000 * t34815;
    let t119144 = t1339 * t9461 * t26508;
    let t119149 = t9446 * t3973 * t34714;
    (t119124, t119128, t119130, t119131, t119141, t119144, t119149)
}
