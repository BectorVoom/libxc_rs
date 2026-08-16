//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta652 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2449;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2450;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta652(t3131: f64, t221: f64, t339: f64, t42813: f64, t10216: f64, t2978: f64, t10479: f64, t42333: f64, t10922: f64, t2960: f64, t1041: f64, t10868: f64, t248: f64, t2776: f64, t3061: f64, t676: f64, t2771: f64, t3129: f64, t42742: f64, t3078: f64, t3082: f64, t3089: f64, t1058: f64, t3068: f64, t3087: f64, t363: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t43292, t43307, t43317, t43322, t43325, t43336) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2449(t3131, t221, t339, t42813, t10216, t2978, t10479, t42333, t10922, t2960, t1041, t10868, t248, t2776);
        let (t43338, t43341, t43343, t43352, t43354, t43358) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2450(t3061, t676, t1041, t248, t2771, t3129, t42742, t3078, t3082, t3089, t1058, t3068, t3087, t363);
    (t43292, t43307, t43317, t43322, t43325, t43336, t43338, t43341, t43343, t43352, t43354, t43358)
}
