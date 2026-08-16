//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta348 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1392;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1393;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta348(t2970: f64, t4522: f64, t973: f64, t10254: f64, t3961: f64, t10236: f64, t10189: f64, t1597: f64, t2990: f64, t2986: f64, t2987: f64, t4540: f64, t2989: f64, t3966: f64, t2960: f64, t4506: f64, t10224: f64, t1592: f64, t4528: f64, t1599: f64, t698: f64, t135: f64, t4542: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13830, t13835, t13839, t13847, t13850, t13851) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1392(t2970, t4522, t973, t10254, t3961, t10236, t10189, t1597, t2990, t2986, t2987, t4540);
        let (t13861, t13893, t13896, t13907, t13909, t13913) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1393(t2989, t3966, t2960, t4506, t10224, t1592, t973, t4528, t1599, t698, t135, t4542);
    (t13830, t13835, t13839, t13847, t13850, t13851, t13861, t13893, t13896, t13907, t13909, t13913)
}
