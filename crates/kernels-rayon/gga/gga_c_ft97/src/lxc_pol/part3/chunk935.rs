//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 935/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk935(t5172: f64, t8392: f64, t5167: f64, t4917: f64, t713: f64, t3892: f64, t9803: f64, t13839: f64, t3887: f64, t1160: f64, t2486: f64, t3893: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18455 = t8392 * t5172;
    let t18457 = t8392 * t5167;
    let t18459 = t4917 * t713;
    let t18460 = t3892 * t18459;
    let t18461 = t9803 * t18460;
    let t18464 = t13839 * t3887;
    let t18467 = t2486 * t1160;
    let t18468 = t18467 * t3893;
    (t18455, t18457, t18459, t18461, t18464, t18468)
}
