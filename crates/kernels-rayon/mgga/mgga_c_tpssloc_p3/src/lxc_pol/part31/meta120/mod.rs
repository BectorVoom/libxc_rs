//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta120 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk674;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk675;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk676;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta120(t3061: f64, t61: f64, t363: f64, t368: f64, t1017: f64, t67: f64, t1058: f64, t1044: f64, t820: f64, t374: f64, t376: f64, t677: f64, t370: f64, t1032: f64, t1036: f64, t121: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3062, t3067, t3068, t3069, t3070) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk674(t3061, t61, t363, t368, t1017, t67, t1058);
        let t3071 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk675(t1044, t820);
        let (t3082, t3084, t3092, t3101) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk676(t374, t376, t677, t370, t1032, t1036, t121);
    (t3062, t3067, t3068, t3069, t3070, t3071, t3082, t3084, t3092, t3101)
}
