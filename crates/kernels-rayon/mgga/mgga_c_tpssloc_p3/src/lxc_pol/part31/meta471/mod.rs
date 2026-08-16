//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta471 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1630;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1631;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta471(t26193: f64, t6907: f64, t1985: f64, t225: f64, t5318: f64, t567: f64, t214: f64, t1377: f64, t1842: f64, t1307: f64, t22635: f64, t22633: f64, t254: f64, t563: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t26206, t26207, t26210, t26211, t26212, t26215, t26216, t26217) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1630(t26193, t6907, t1985, t225, t5318, t567, t214, t1377, t1842, t1307, t22635, t22633);
        let t26224 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1631(t254, t563);
    (t26206, t26207, t26210, t26211, t26212, t26215, t26216, t26217, t26224)
}
