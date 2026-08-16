//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1319/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1319(t1983: f64, t31221: f64, t5161: f64, t12461: f64, t8488: f64, t26161: f64, t26163: f64, t114360: f64, t25971: f64, t33129: f64, t6876: f64, t32670: f64, t4034: f64) -> (f64, f64, f64, f64, f64) {
    let t120097 = t1983 * t31221 * t5161;
    let t120100 = t8488 * t12461;
    let t120103 = 2.0_f64 * t26161 * t120100 * t26163;
    let t120104 = t114360 * t25971;
    let t120107 = 3.0_f64 * t6876 * t33129;
    let t120108 = t4034 * t32670;
    (t120097, t120103, t120104, t120107, t120108)
}
