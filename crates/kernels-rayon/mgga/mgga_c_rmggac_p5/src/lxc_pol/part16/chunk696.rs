//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 696/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk696(t1737: f64, t645: f64, t4044: f64, t2344: f64, t8659: f64, t2329: f64, t8365: f64, t209: f64, t605: f64, t615: f64, t236: f64, t1971: f64) -> (f64, f64, f64, f64, f64) {
    let t10053 = t645 * t1737;
    let t10054 = t4044 * t10053;
    let t10056 = t8659 * t2344;
    let t10058 = t8365 * t2329;
    let t10064 = t615 * t605 * t209;
    let t10065 = t236 * t10064;
    let t10066 = t1971 * t10065;
    (t10053, t10054, t10056, t10058, t10066)
}
