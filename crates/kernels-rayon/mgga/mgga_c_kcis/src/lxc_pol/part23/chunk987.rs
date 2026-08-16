//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 987/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk987(t1599: f64, t18163: f64, t12844: f64, t6155: f64, t4439: f64, t3970: f64, t617: f64) -> (f64, f64, f64) {
    let t18164 = t1599 * t18163;
    let t18168 = t12844 * t6155;
    let t18170 = t4439 * t18168 / 864.0_f64;
    let t18171 = t3970 * t617;
    (t18164, t18170, t18171)
}
