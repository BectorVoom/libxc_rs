//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1368/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1368(t10922: f64, t973: f64, t2960: f64, t3139: f64, t1030: f64, t363: f64, t3068: f64, t1058: f64, t3030: f64, t990: f64, t3032: f64, t3129: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10923 = t973 * t10922;
    let t10927 = t2960 * t3139;
    let t10935 = t363 * t1030;
    let t10936 = t10935 * t3068;
    let t10937 = t1058 * t10936;
    let t10947 = t990 * t3030;
    let t10948 = t10947 * t3032;
    let t10949 = t10948 * t3129;
    (t10923, t10927, t10937, t10947, t10948, t10949)
}
