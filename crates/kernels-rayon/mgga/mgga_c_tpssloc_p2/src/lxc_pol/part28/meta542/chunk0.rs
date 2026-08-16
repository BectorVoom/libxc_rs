//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1806/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1806(t22813: f64, t6589: f64, t80782: f64, t23124: f64, t23083: f64, t23086: f64, t23138: f64, t6604: f64, t6606: f64, t22690: f64, t2627: f64, t236: f64, t2631: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t81902 = t22813 * t6589 * t80782;
    let t81903 = t81902 * t23124;
    let t81909 = t23083 * t23086;
    let t81911 = t23138 * t6604;
    let t81912 = t81911 * t6606;
    let t81914 = t22690 * t2627;
    let t81915 = t236 * t2631;
    (t81902, t81903, t81909, t81911, t81912, t81914, t81915)
}
