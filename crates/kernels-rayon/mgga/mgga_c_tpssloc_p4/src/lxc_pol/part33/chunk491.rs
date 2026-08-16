//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 491/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk491(t2978: f64, t974: f64, t2770: f64, t344: f64, t337: f64, t39: f64, t1887: f64, t60: f64, t976: f64) -> (f64, f64, f64, f64) {
    let t2979 = t974 * t2978;
    let t2980 = t344 * t2770;
    let t2985 = t39 * t337;
    let t2986 = t2985 * t1887;
    let t2987 = t60 * t976;
    (t2979, t2980, t2986, t2987)
}
