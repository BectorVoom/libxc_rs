//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 301/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk301(t339: f64, t995: f64, t883: f64, t976: f64, t607: f64, t974: f64) -> (f64, f64, f64, f64) {
    let t997 = t339 * t995 / 288.0_f64;
    let t998 = t976 * t883;
    let t999 = t998 * t607;
    let t1000 = t974 * t999;
    (t997, t998, t999, t1000)
}
