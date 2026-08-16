//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 935/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk935(t11002: f64, t3130: f64, t1025: f64, t1041: f64, t1046: f64, t10932: f64, t10937: f64, t10944: f64, t10949: f64, t10952: f64, t10957: f64, t10962: f64, t10965: f64, t10972: f64, t10982: f64, t10985: f64, t10988: f64, t10994: f64, t10998: f64, t2960: f64, t3043: f64, t3048: f64, t3057: f64, t3064: f64, t3073: f64, t3117: f64, t3134: f64, t3143: f64, t3148: f64, t3153: f64, t973: f64) -> f64 {
    let t11003 = t3130 * t11002;
    let t11005 = -t973 * t10932 / 36.0_f64 - t10937 * t3073 / 144.0_f64 + 5.0_f64 / 4608.0_f64 * t3117 * t3064 + 7.0_f64 / 648.0_f64 * t973 * t10944 + t10949 * t3134 / 512.0_f64 - t10952 * t3043 / 1024.0_f64 + 19.0_f64 / 864.0_f64 * t10957 * t1046 + t10962 * t1025 / 1024.0_f64 + t10965 * t1046 / 1536.0_f64 + 5.0_f64 / 5184.0_f64 * t1041 * t10972 - t3048 * t3057 / 288.0_f64 - t2960 * t3143 / 36.0_f64 - t2960 * t3148 / 27.0_f64 + t10982 / 288.0_f64 + t10985 / 216.0_f64 + t973 * t10988 / 288.0_f64 + t2960 * t3153 / 18.0_f64 - t10994 / 144.0_f64 + t973 * t10998 / 48.0_f64 + t11003 / 768.0_f64;
    t11005
}
