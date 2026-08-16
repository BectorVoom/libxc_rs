//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 912/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk912(t5148: f64, t74811: f64, t15075: f64, t30526: f64, t3851: f64, t75886: f64, t75216: f64, t793: f64, t41400: f64, t649: f64, t8950: f64, t40932: f64, t8979: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t76326 = 0.5987120850931904282e-1_f64 * t5148 * t74811;
    let t76331 = t30526 * t15075;
    let t76333 = t3851 * t75886;
    let t76337 = t793 * t75216;
    let t76340 = t41400 * t649 * t8950;
    let t76343 = t40932 * t649 * t8979;
    (t76326, t76331, t76333, t76337, t76340, t76343)
}
