//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 973/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk973<F: Float>(t322: F, t1079: F, t1305: F, t1081: F, t1312: F, t1310: F, t3386: F, t839: F, t11059: F) -> (F, F, F, F, F) {
    let t332 = F::cast_from(0.25e1_f64) < t322;
    let t11087 = t1079 * t1305;
    let t11092 = t1312 * t1081;
    let t11106 = t1310 * t1081;
    let t11108 = t839 * t3386;
    let t11110 = piecewise3::<F>(t332, F::cast_from(0.0_f64), t11059);
    (t11087, t11092, t11106, t11108, t11110)
}
