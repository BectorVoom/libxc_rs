//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 753/1310 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk753(t1625: f64, t198: f64, t8820: f64, t1622: f64, t1043: f64, t1674: f64, t8778: f64, t8782: f64, t8790: f64, t8794: f64, t8796: f64, t8799: f64, t8802: f64, t8805: f64, t8811: f64, t8815: f64, t8818: f64) -> (f64, f64) {
    let t8822 = t8820 * t198 * t1625;
    let t8823 = t1622 * t8822;
    let t8825 = t1043 * t1674;
    let t8827 = -0.56275309320814680968e-8_f64 * t8778 - 0.11255061864162936194e-7_f64 * t8782 + 0.82068159426188076412e-9_f64 * t8790 - 0.5627530932081468097e-7_f64 * t8794 + 0.44316806090141561263e-6_f64 * t8796 - 0.51585700210746790888e-5_f64 * t8799 + 0.12163329537032409896e-2_f64 * t8802 - 0.20241536458333333334e-4_f64 * t8805 - 0.41193142698749761516e-5_f64 * t8811 + 0.67471788194444444446e-5_f64 * t8815 + 0.13900948042322754167e-2_f64 * t8818 + 0.1374296967252737644e-6_f64 * t8823 - 0.33816362383187442026e-4_f64 * t8825;
    (t8822, t8827)
}
