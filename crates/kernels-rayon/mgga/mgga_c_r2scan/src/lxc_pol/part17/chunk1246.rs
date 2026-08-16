//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1246/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1246(t38183: f64, t38666: f64, t41775: f64, t41776: f64, t43688: f64, t43690: f64, t43692: f64, t43695: f64, t43697: f64, t43700: f64, t43702: f64, t43705: f64) -> f64 {
    let t44510 = -0.10975748638225852664e0_f64 * t43688 + 0.17336443480108537126e0_f64 * t43690 + 0.5854464323629669811e-1_f64 * t43692 - 0.32927245914677557993e-1_f64 * t38183 + t38666 + t41775 - 0.25610080155860322883e0_f64 * t43695 - 0.86682217400542685632e-1_f64 * t43697 - 0.86682217400542685632e-1_f64 * t43700 - 0.86682217400542685632e-1_f64 * t43702 - t41776 + 0.13099107994629972538e-1_f64 * t43705;
    t44510
}
