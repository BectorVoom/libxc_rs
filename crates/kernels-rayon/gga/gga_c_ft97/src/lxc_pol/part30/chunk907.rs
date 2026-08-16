//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 907/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk907(t10697: f64, t848: f64, t319: f64, t43912: f64, t2680: f64, t309: f64, t43917: f64, t799: f64, t863: f64, t2766: f64, t2843: f64, t10491: f64, t1240: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t56352 = t848 * t10697;
    let t56418 = t43912 * t319;
    let t56456 = t2680 * t309;
    let t56643 = t43917 * t319;
    let t56815 = t799 * t863;
    let t56819 = t2766 * t2843;
    let t57089 = t10491 * t1240;
    (t56352, t56418, t56456, t56643, t56815, t56819, t57089)
}
