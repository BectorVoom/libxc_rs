//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1110/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1110(t573: f64, t5998: f64, t27517: f64, t8196: f64, t1468: f64, t5929: f64, t2062: f64, t3738: f64, t5910: f64, t7952: f64, t27543: f64, t4122: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t28614 = t5998 * t573;
    let t28616 = t27517 * t8196;
    let t28618 = t1468 * t5929;
    let t28620 = t3738 * t2062;
    let t28622 = t7952 * t5910;
    let t28624 = t4122 * t27543;
    (t28614, t28616, t28618, t28620, t28622, t28624)
}
