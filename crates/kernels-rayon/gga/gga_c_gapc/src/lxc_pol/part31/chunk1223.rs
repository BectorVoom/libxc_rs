//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1223/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1223(t21643: f64, t34363: f64, t21801: f64, t5395: f64, t5743: f64, t1743: f64, t5722: f64, t1030: f64, t33311: f64, t3714: f64, t1036: f64, t11316: f64, t13483: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t34364 = t34363 * t21643;
    let t34366 = t5395 * t21801;
    let t34367 = t34366 * t5743;
    let t34370 = t1743 * t21801 * t5722;
    let t34372 = t1030 * t33311;
    let t34373 = t34372 * t3714;
    let t34378 = t11316 * t1036 * t13483;
    (t34364, t34366, t34367, t34370, t34372, t34373, t34378)
}
