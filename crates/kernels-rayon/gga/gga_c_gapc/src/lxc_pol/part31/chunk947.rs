//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 947/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk947(t9636: f64, t9639: f64, t9642: f64, t9646: f64, t9649: f64, t9653: f64, t9656: f64, t9659: f64, t9662: f64, t9665: f64, t9668: f64, t9671: f64, t9674: f64) -> f64 {
    let t10901 = -0.10120442708333333334e-4_f64 * t9636 - 0.17376185052903442709e-3_f64 * t9639 + 0.28960308421505737848e-5_f64 * t9642 + 0.42233783114695867695e-6_f64 * t9646 - 0.2318836277704281739e-4_f64 * t9649 + 0.56273499301538336858e-8_f64 * t9653 + 0.56273499301538336858e-8_f64 * t9656 - 0.55603792169291016668e-2_f64 * t9659 + 0.24326659074064819792e-2_f64 * t9662 - 0.55603792169291016668e-2_f64 * t9665 + 0.18550690221634253912e-3_f64 * t9668 - 0.10005428175813516294e-7_f64 * t9671 - 0.51584041026410142121e-5_f64 * t9674;
    t10901
}
