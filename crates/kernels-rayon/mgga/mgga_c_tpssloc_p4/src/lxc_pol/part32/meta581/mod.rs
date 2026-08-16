//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta581 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1963;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1964;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta581(t2123: f64, t6146: f64, t2144: f64, t6150: f64, t1720: f64, t8054: f64, t5971: f64, t7286: f64, t24595: f64, t27426: f64, t8002: f64, t2121: f64, t2124: f64, t27755: f64, t27770: f64, t29671: f64, t29674: f64, t29678: f64, t498: f64, t7283: f64, t7999: f64, t8011: f64, t2148: f64, t6140: f64, t6224: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t29682, t29685, t29687, t29690, t29691, t29694, t29699) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1963(t2123, t6146, t2144, t6150, t1720, t8054, t5971, t7286, t24595, t27426, t8002, t2121, t2124, t27755, t27770, t29671, t29674, t29678, t498, t7283, t7999, t8011);
        let (t29702, t29705, t29708) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1964(t2148, t6146, t6140, t2144, t6224);
    (t29682, t29685, t29687, t29690, t29691, t29694, t29699, t29702, t29705, t29708)
}
