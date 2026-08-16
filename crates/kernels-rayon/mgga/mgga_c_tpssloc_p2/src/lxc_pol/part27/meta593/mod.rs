//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta593 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2054;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2055;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta593(t23083: f64, t23086: f64, t23138: f64, t6604: f64, t6606: f64, t22690: f64, t2627: f64, t236: f64, t2631: f64, t23109: f64, t2632: f64, t10024: f64, t1899: f64, t23110: f64, t232: f64, t23116: f64, t838: f64, t2693: f64, t6609: f64, t213: f64, t6589: f64, t9223: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t81909, t81911, t81912, t81914, t81915, t81918, t81920) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2054(t23083, t23086, t23138, t6604, t6606, t22690, t2627, t236, t2631, t23109, t2632, t10024, t1899);
        let (t81921, t81924, t81926, t81928, t81933) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2055(t81920, t23109, t23110, t232, t81915, t23116, t838, t2693, t6609, t213, t6589, t9223);
    (t81909, t81911, t81912, t81914, t81918, t81921, t81924, t81926, t81928, t81933)
}
