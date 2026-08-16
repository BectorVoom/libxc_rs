//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 706/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk706(t10143: f64, t1914: f64, t221: f64, t2987: f64, t1926: f64, t344: f64, t381: f64, t225: f64, t1054: f64, t883: f64, t1922: f64, t2966: f64) -> (f64, f64, f64, f64, f64) {
    let t23295 = t1914 * t10143;
    let t23326 = t221 * t2987;
    let t23327 = t1926 * t23326;
    let t23328 = t344 * t381;
    let t23329 = t23328 * t225;
    let t23330 = t1054 * t883;
    let t23357 = t2966 * t1922;
    (t23295, t23327, t23329, t23330, t23357)
}
