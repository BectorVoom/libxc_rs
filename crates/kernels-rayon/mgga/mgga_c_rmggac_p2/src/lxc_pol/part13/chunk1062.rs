//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1062/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1062(t39840: f64, t39842: f64, t333: f64, t9565: f64, t35407: f64, t35413: f64, t35424: f64, t39813: f64, t39818: f64, t39830: f64, t39833: f64, t39838: f64, t39855: f64, t39859: f64, t39861: f64, t39864: f64, t39869: f64, t4041: f64, t884: f64, t9405: f64) -> (f64, f64) {
    let t43157 = 0.49658699875514145965e-4_f64 * t39840;
    let t43158 = 0.11918087970123395032e-3_f64 * t39842;
    let t43163 = t9565 * t333;
    let t43167 = -0.23948483403727617128e0_f64 * t4041 * t9405 + 0.15323255961587222184e-3_f64 * t39813 + 0.5107751987195740728e-4_f64 * t39818 - 0.95793933614910468511e0_f64 * t35407 - 0.3193131120497015617e0_f64 * t35413 - 0.36366215538993788974e0_f64 * t35424 - 0.85129199786595678799e-5_f64 * t39830 - 0.1702583995731913576e-4_f64 * t39833 - 0.23942587439980034662e-4_f64 * t39838 - t43157 + t43158 - 0.3405167991463827152e-4_f64 * t39855 + 0.3405167991463827152e-4_f64 * t39859 + 0.5107751987195740728e-4_f64 * t39861 + 0.68186654135613354325e-2_f64 * t39864 + 0.11974241701863808564e0_f64 * t884 * t43163 - 0.212822999466489197e-4_f64 * t39869;
    (t43163, t43167)
}
