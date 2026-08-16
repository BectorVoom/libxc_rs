//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1344/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1344(t1005: f64, t3082: f64, t121: f64, t3061: f64, t1008: f64) -> (f64, f64, f64, f64) {
    let t10436 = t1005 * t3082;
    let t10457 = t121 * t3061;
    let t10468 = t1008 * t1008;
    let t10469 = 1.0_f64 / t10468;
    (t10436, t10457, t10468, t10469)
}
