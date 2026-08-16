//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 989/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk989(t11725: f64, t11728: f64, t11730: f64, t11732: f64, t11734: f64, t11737: f64, t11739: f64, t11742: f64, t11745: f64, t11749: f64, t11751: f64, t12158: f64) -> f64 {
    let t12159 = -0.43663693315433241794e-2_f64 * t11725 + 0.69345773920434148507e0_f64 * t11728 + 0.25610080155860322883e0_f64 * t11730 - 0.10975748638225852664e0_f64 * t11732 - 0.86682217400542685632e-1_f64 * t11734 - 0.86682217400542685632e-1_f64 * t11737 - 0.2600466522016280569e0_f64 * t11739 - 0.2600466522016280569e0_f64 * t11742 - 0.86682217400542685632e-1_f64 * t11745 - 0.2600466522016280569e0_f64 * t11749 + 0.10975748638225852664e0_f64 * t11751 - t12158;
    t12159
}
