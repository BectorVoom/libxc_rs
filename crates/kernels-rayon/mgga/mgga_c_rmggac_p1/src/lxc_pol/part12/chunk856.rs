//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 856/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk856(t1607: f64, t1986: f64, t7720: f64, t7279: f64, t8365: f64, t35906: f64, t570: f64, t1979: f64, t1982: f64, t201: f64, t597: f64, t998: f64) -> (f64, f64, f64, f64) {
    let t38943 = t1986 * t1607;
    let t38944 = t7720 * t38943;
    let t38946 = t8365 * t7279;
    let t38948 = t35906 * t570;
    let t38958 = t998 * t597 * t201 * t1979 * t1982;
    (t38944, t38946, t38948, t38958)
}
