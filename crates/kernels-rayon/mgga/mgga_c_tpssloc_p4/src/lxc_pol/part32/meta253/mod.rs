//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta253 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1144;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1145;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1146;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta253(t28: f64, t776: f64, t868: f64, t1081: f64, t1877: f64, t1915: f64, t2522: f64, t6666: f64, t6670: f64, t1873: f64, t2314: f64, t5113: f64, t1268: f64, t6534: f64, t1271: f64, t191: f64, t192: f64, t2020: f64, t2018: f64, t532: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t6841, t6848, t6855, t6867, t6869) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1144(t28, t776, t868, t1081, t1877, t1915, t2522, t6666, t6670, t1873, t2314, t5113);
        let (t6871, t6875, t6876) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1145(t1268, t6534, t1271, t191, t192);
        let (t6877, t6878) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1146(t2020, t6876, t2018, t532);
    (t6841, t6848, t6855, t6867, t6869, t6871, t6875, t6876, t6877, t6878)
}
