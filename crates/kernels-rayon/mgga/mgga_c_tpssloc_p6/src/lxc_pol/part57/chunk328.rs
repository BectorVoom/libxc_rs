//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 328/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk328(t225: f64, t562: f64, t567: f64, t214: f64, t1985: f64, t1878: f64, t1887: f64, t534: f64) -> (f64, f64, f64, f64) {
    let t1987 = t562 * t225 * t567;
    let t1988 = t214 * t1987;
    let t1989 = t1985 * t1988;
    let t1992 = t1878 * t534 * t1887;
    (t1987, t1988, t1989, t1992)
}
