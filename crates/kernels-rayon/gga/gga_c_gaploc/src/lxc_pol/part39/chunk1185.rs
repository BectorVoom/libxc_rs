//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1185/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1185(t12065: f64, t2441: f64, t38759: f64, t895: f64, t10348: f64, t12054: f64, t41885: f64, t41889: f64, t41891: f64, t41893: f64, t41897: f64, t41900: f64, t41904: f64, t41905: f64, t41907: f64) -> f64 {
    let t47937 = t2441 * t12065;
    let t47939 = t895 * t38759;
    let t47941 = t12054 * t10348;
    let t47946 = 0.35750489951850426669e0_f64 * t47937 + 0.35750489951850426669e0_f64 * t47939 - 0.7150097990370085334e0_f64 * t47941 + t41885 - t41889 - 0.19171462976960374838e0_f64 * t41891 + t41893 + 0.11916829983950142223e0_f64 * t41897 + 0.19171462976960374838e0_f64 * t41900 + t41904 - t41905 + t41907;
    t47946
}
