//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1353/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1353(t10423: f64, t10937: f64, t2955: f64, t3158: f64, t10383: f64, t964: f64, t1020: f64, t10508: f64, t248: f64, t3121: f64, t10949: f64, t11002: f64) -> (f64, f64, f64, f64, f64) {
    let t43143 = t10937 * t10423;
    let t43155 = t2955 * t3158;
    let t43157 = t964 * t10383;
    let t43161 = t1020 * t248 * t10508 * t3121;
    let t43167 = t10949 * t11002;
    (t43143, t43155, t43157, t43161, t43167)
}
