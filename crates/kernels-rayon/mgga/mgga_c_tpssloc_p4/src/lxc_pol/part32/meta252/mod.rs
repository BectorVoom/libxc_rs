//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta252 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1141;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1142;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1143;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta252(t25: f64, t868: f64, t1877: f64, t1915: f64, t2522: f64, t606: f64, t6542: f64, t6666: f64, t6670: f64, t221: f64, t60: f64, t3: f64, t607: f64, t343: f64, t984: f64, t3034: f64, t334: f64, t371: f64, t202: f64, t6665: f64, t193: f64, t776: f64, t870: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t6671, t6678, t6686, t6729) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1141(t25, t868, t1877, t1915, t2522, t606, t6542, t6666, t6670, t221, t60, t3, t607);
        let (t6733, t6739) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1142(t343, t984, t3034, t334);
        let (t6793, t6794, t6834) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1143(t334, t371, t202, t6665, t1877, t1915, t193, t2522, t6670, t776, t868, t870);
    (t6671, t6678, t6686, t6729, t6733, t6739, t6793, t6794, t6834)
}
