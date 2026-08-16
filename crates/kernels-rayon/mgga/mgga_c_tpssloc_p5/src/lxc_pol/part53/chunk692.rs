//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 692/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk692(t8343: f64, t8344: f64, t1268: f64, t8326: f64, t1998: f64, t59: f64) -> (f64, f64, f64) {
    let t8345 = t8343 * t8344;
    let t8445 = t1268 * t8326;
    let t8446 = 2.0_f64 * t8445;
    let t8462 = t1998 * t59;
    (t8345, t8446, t8462)
}
