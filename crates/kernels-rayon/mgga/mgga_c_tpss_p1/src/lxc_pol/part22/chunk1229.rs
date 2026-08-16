//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1229/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1229(t18947: f64, t219: f64, t5919: f64, t1838: f64, t18490: f64, t3366: f64, t1219: f64, param_beta: f64) -> (f64, f64, f64, f64) {
    let t18948 = param_beta * t18947;
    let t18950 = t5919 * t219;
    let t18964 = t18490 * t1838 * t3366;
    let t18967 = t1219 * t1838;
    (t18948, t18950, t18964, t18967)
}
