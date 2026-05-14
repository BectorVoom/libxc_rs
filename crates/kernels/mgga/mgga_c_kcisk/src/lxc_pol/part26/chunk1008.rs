//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1008/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1008<F: Float>(t3764: F, t7744: F, t1340: F, t1339: F, t12841: F, t8094: F, t1440: F, t7906: F, t1341: F, t3785: F, t1411: F, t1219: F, t7828: F) -> (F, F, F, F, F) {
    let t27004 = t3764 * t7744;
    let t27005 = t1340 * t27004;
    let t27006 = t1339 * t27005;
    let t27008 = t12841 * t8094;
    let t27010 = t7906 * t1440;
    let t27011 = t1341 * t27010;
    let t27012 = t3785 * t27011;
    let t27013 = t1411 * t27012;
    let t27016 = t7828 * t1219;
    (t27006, t27008, t27010, t27013, t27016)
}
