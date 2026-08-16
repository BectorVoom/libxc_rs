//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 844/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk844(t7244: f64, t9159: f64, t1971: f64, t3351: f64, t5156: f64, t7190: f64, t1607: f64, t1986: f64, t7720: f64, t7279: f64, t8365: f64, t1979: f64, t1982: f64, t201: f64, t597: f64, t998: f64) -> (f64, f64, f64, f64, f64) {
    let t38934 = t7244 * t9159;
    let t38938 = t3351 * t1971 * t7190 * t5156;
    let t38943 = t1986 * t1607;
    let t38944 = t7720 * t38943;
    let t38946 = t8365 * t7279;
    let t38958 = t998 * t597 * t201 * t1979 * t1982;
    (t38934, t38938, t38944, t38946, t38958)
}
