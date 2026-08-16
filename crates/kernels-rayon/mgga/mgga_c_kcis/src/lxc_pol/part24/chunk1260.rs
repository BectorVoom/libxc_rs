//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1260/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1260(t1774: f64, t303: f64, t4924: f64, t100078: f64, t100578: f64, t100580: f64, t100583: f64, t100586: f64, t100596: f64, t100599: f64, t11072: f64, t1268: f64, t26960: f64, t28098: f64, t6774: f64, t922: f64, t96917: f64, t97193: f64) -> (f64, f64) {
    let t100602 = t303 * t4924 * t1774;
    let t100606 = 0.11607361111111111111e-2_f64 * t100578 - t97193 - 0.11607361111111111111e-2_f64 * t100580 - 0.30952962962962962962e-2_f64 * t100583 + 0.25794135802469135802e-2_f64 * t100586 - 0.23168402777777777778e-3_f64 * t26960 * t11072 * t1268 * t6774 * t922 + 0.23168402777777777778e-3_f64 * t96917 * t28098 + 0.11607361111111111111e-2_f64 * t100596 + 0.11607361111111111111e-2_f64 * t100599 + 0.23214722222222222222e-2_f64 * t100602 + 0.11584201388888888889e-3_f64 * t26960 * t100078;
    (t100602, t100606)
}
