//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta742 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2608;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2609;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta742(t10477: f64, t1742: f64, t11713: f64, t3503: f64, t1210: f64, t11719: f64, t13969: f64, t15626: f64, t11529: f64, t1174: f64, t4729: f64, t11647: f64, t1731: f64, t1227: f64, t15616: f64, t14706: f64, t248: f64, t3521: f64, t11814: f64, t4997: f64, t15492: f64, t3536: f64, t11692: f64, t11697: f64, t15703: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t53081, t53083, t53087, t53093, t53096, t53099) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2608(t10477, t1742, t11713, t3503, t1210, t11719, t13969, t15626, t11529, t1174, t4729, t11647, t1731);
        let (t53102, t53114, t53116, t53118, t53135) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2609(t1227, t13969, t15616, t14706, t248, t3521, t11814, t4997, t15492, t3536, t11692, t11697, t15703);
    (t53081, t53083, t53087, t53093, t53096, t53099, t53102, t53114, t53116, t53118, t53135)
}
