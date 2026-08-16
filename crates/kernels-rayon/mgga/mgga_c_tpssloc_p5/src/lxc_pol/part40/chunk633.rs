//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 633/1303 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk633(t1369: f64, t3866: f64, t1995: f64, t241: f64, t67: f64, t1373: f64, t225: f64, t1376: f64, t566: f64, t68: f64, t3787: f64, t562: f64) -> (f64, f64, f64, f64, f64) {
    let t3867 = t3866 * t1369;
    let t3869 = t241 * t1995;
    let t3870 = t3869 * t67;
    let t3882 = t1373 * t225;
    let t3886 = 1.0_f64 / t1376 / t566;
    let t3887 = t68 * t3886;
    let t3897 = t3787 * t562;
    (t3867, t3870, t3882, t3887, t3897)
}
