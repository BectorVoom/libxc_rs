//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta271 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1268;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1269;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta271(t1530: f64, t28: f64, t1649: f64, t1877: f64, t1915: f64, t2522: f64, t6670: f64, t7541: f64, t7650: f64, t1873: f64, t4028: f64, t1458: f64, t88: f64, t1268: f64, t7467: f64, t1778: f64, t191: f64, t192: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t7656, t7663, t7675, t7676) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1268(t1530, t28, t1649, t1877, t1915, t2522, t6670, t7541, t7650, t1873, t4028, t1458, t88);
        let (t7678, t7680, t7684, t7685) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1269(t1873, t7676, t1268, t7467, t1778, t191, t192);
    (t7656, t7663, t7675, t7676, t7678, t7680, t7684, t7685)
}
