//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta307 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1226;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1227;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta307(t10922: f64, t973: f64, t2960: f64, t3139: f64, t1030: f64, t363: f64, t3068: f64, t1058: f64, t3030: f64, t990: f64, t3032: f64, t3129: f64, t3038: f64, t3087: f64, t372: f64, t364: f64, t354: f64, t1009: f64, t3020: f64, t1011: f64, t1019: f64, t1040: f64, t3077: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10923, t10927, t10937, t10947, t10948, t10949) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1226(t10922, t973, t2960, t3139, t1030, t363, t3068, t1058, t3030, t990, t3032, t3129);
        let (t10952, t10957, t10960, t10962, t10965) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1227(t10948, t3038, t3087, t372, t364, t354, t1009, t3020, t1011, t1019, t1040, t3077);
    (t10923, t10927, t10937, t10947, t10949, t10952, t10957, t10960, t10962, t10965)
}
