//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1185/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1185(t11670: f64, t2124: f64, t29764: f64, t11705: f64, t7313: f64, t11708: f64, t8240: f64, t12538: f64, t6395: f64, t10868: f64, t2147: f64, t9292: f64) -> (f64, f64, f64, f64, f64) {
    let t43512 = t11670 * t2124 * t29764;
    let t43514 = t7313 * t11705;
    let t43516 = t8240 * t11708;
    let t43518 = t6395 * t12538;
    let t43521 = t2147 * t10868 * t9292;
    (t43512, t43514, t43516, t43518, t43521)
}
