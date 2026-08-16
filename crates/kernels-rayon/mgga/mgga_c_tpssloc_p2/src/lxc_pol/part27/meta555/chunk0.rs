//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1996/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1996(t25962: f64, t25999: f64, t26155: f64, t26507: f64, t3: f64, t112: f64, t7758: f64, t16521: f64, t1873: f64, t16524: f64, t7015: f64, t5371: f64, t6534: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t26509 = t25962 + t25999 + t26155 + t26507;
    let t26510 = t3 * t26509;
    let t26523 = t7758 * t112;
    let t26533 = 0.135e2_f64 * t16521 * t1873;
    let t26535 = 27.0_f64 * t16524 * t7015;
    let t26537 = 0.135e2_f64 * t5371 * t6534;
    (t26509, t26510, t26523, t26533, t26535, t26537)
}
