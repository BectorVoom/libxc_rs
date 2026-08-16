//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1211/1427 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1211(t11302: f64, t19844: f64, t5974: f64, t1743: f64, t33148: f64, t19644: f64, t11356: f64, t9071: f64, t9256: f64, t11604: f64, t26836: f64, t11468: f64, t3065: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34315 = t19844 * t11302 * t5974;
    let t34317 = t1743 * t33148;
    let t34318 = t34317 * t19644;
    let t34321 = t9071 * t11356 * t9256;
    let t34323 = t11604 * t26836;
    let t34325 = t11468 * t3065;
    (t34315, t34317, t34318, t34321, t34323, t34325)
}
