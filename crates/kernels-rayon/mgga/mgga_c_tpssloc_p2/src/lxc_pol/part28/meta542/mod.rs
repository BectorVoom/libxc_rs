//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta542 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1806;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1807;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta542(t22813: f64, t6589: f64, t80782: f64, t23124: f64, t23083: f64, t23086: f64, t23138: f64, t6604: f64, t6606: f64, t22690: f64, t2627: f64, t236: f64, t2631: f64, t23109: f64, t2632: f64, t10024: f64, t1899: f64, t23110: f64, t232: f64, t23116: f64, t838: f64, t2693: f64, t6609: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t81902, t81903, t81909, t81911, t81912, t81914, t81915) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1806(t22813, t6589, t80782, t23124, t23083, t23086, t23138, t6604, t6606, t22690, t2627, t236, t2631);
        let (t81918, t81920, t81924, t81926, t81928) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1807(t23109, t2632, t81914, t81915, t10024, t1899, t23110, t232, t23116, t838, t2693, t6609);
    (t81902, t81903, t81909, t81911, t81912, t81914, t81918, t81920, t81924, t81926, t81928)
}
