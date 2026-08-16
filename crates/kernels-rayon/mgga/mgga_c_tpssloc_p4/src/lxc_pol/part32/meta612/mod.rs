//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta612 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2011;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2012;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta612(t22813: f64, t6589: f64, t80782: f64, t23124: f64, t23138: f64, t6604: f64, t6606: f64, t22690: f64, t2627: f64, t10024: f64, t1899: f64, t2693: f64, t6609: f64, t213: f64, t9223: f64, t6593: f64, t22715: f64, t229: f64, t805: f64, t1891: f64, t192: f64, t80881: f64, t841: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t81902, t81903, t81911, t81912, t81914, t81921, t81928) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2011(t22813, t6589, t80782, t23124, t23138, t6604, t6606, t22690, t2627, t10024, t1899, t2693, t6609);
        let (t81933, t81934, t81942, t81943, t81954) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2012(t213, t6589, t9223, t6593, t22715, t229, t805, t1891, t192, t22690, t80881, t841);
    (t81902, t81903, t81911, t81912, t81914, t81921, t81928, t81933, t81934, t81942, t81943, t81954)
}
