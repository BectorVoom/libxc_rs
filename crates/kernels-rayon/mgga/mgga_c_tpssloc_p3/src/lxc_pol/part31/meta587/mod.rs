//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta587 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1829;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1830;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta587(t26322: f64, t80855: f64, t91152: f64, t236: f64, t26318: f64, t91005: f64, t22782: f64, t5234: f64, t1369: f64, t7712: f64, t80939: f64, t22683: f64, t26285: f64, t6546: f64, t26289: f64, t6604: f64, t80887: f64, t16060: f64, t6951: f64, t1878: f64, t80730: f64, t80893: f64, t6925: f64, t6976: f64, t26271: f64, t80779: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t91154, t91158, t91160, t91161, t91167, t91170) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1829(t26322, t80855, t91152, t236, t26318, t91005, t22782, t5234, t1369, t7712, t80939, t22683, t26285, t6546);
        let (t91179, t91191, t91194, t91198, t91202, t91206) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1830(t26289, t6604, t80887, t16060, t6951, t1878, t80730, t80893, t6925, t6976, t26271, t80779);
    (t91154, t91158, t91160, t91161, t91167, t91170, t91179, t91191, t91194, t91198, t91202, t91206)
}
