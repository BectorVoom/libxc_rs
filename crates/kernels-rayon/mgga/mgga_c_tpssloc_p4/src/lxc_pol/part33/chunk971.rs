//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 971/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk971(t4483: f64, t5812: f64, t1568: f64, t5742: f64, t2888: f64, t10277: f64, t20234: f64) -> (f64, f64, f64, f64) {
    let t21107 = 0.51947577317044391276e2_f64 * t4483 * t5812;
    let t21114 = t5742 * t1568;
    let t21115 = t21114 * t2888;
    let t21118 = t10277 * t20234;
    (t21107, t21114, t21115, t21118)
}
