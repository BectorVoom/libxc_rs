//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 332/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk332<F: Float>(t1248: F, t1250: F, t482: F, t1042: F, t127: F, t371: F, t481: F, t369: F, t479: F, t475: F) -> (F, F, F, F, F) {
    let t1251 = t482 * t1248 * t1250;
    let t1252 = t1042 * t1251;
    let t1256 = t371 * t127 * t482;
    let t1258 = F::cast_from(0.14291339372689912324e-3_f64) * t481 * t1256;
    let t1259 = t479 * t369;
    let t1260 = t475 * t1259;
    (t1251, t1252, t1256, t1258, t1260)
}
