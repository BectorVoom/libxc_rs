//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1343/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1343(t10225: f64, t2960: f64, t10213: f64, t135: f64, t10218: f64, t973: f64, t344: f64, t41687: f64, t10236: f64, t10913: f64, t41831: f64, t41833: f64, t41836: f64, t41839: f64, t41842: f64, t41887: f64, t41889: f64, t41892: f64, t41964: f64, t41967: f64, t41970: f64) -> (f64, f64, f64, f64, f64) {
    let t42968 = t2960 * t10225;
    let t42972 = t135 * t10213;
    let t42974 = t973 * t42972 * t10218;
    let t42976 = t344 * t41687;
    let t42985 = t10236 * t10913;
    let t43000 = -20.0_f64 / 9.0_f64 * t41831 - 8.0_f64 / 3.0_f64 * t41833 + 8.0_f64 / 3.0_f64 * t41887 - 4.0_f64 / 9.0_f64 * t41889 + 2.0_f64 * t41836 - 2.0_f64 * t41892 + t41839 / 6.0_f64 + 2.0_f64 / 9.0_f64 * t41964 + 4.0_f64 / 9.0_f64 * t41967 - 4.0_f64 * t41842 + 6.0_f64 * t41970;
    (t42968, t42974, t42976, t42985, t43000)
}
