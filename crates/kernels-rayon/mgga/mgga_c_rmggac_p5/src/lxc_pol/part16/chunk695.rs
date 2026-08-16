//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 695/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk695(t10040: f64, t675: f64, t1859: f64, t194: f64, t201: f64, t1979: f64, t1982: f64, t128: f64, t1864: f64, t118: f64, t1986: f64, t7408: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10041 = t675 * t10040;
    let t10043 = t194 * t1859;
    let t10044 = t10043 * t201;
    let t10046 = t10044 * t1979 * t1982;
    let t10048 = t128 * t1864;
    let t10049 = t118 * t10048;
    let t10050 = t1986 * t10049;
    let t10051 = t7408 * t10050;
    (t10041, t10043, t10044, t10046, t10050, t10051)
}
