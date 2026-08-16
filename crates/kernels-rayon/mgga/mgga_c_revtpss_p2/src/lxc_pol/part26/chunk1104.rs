//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1104/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1104(t2061: f64, t2718: f64, t198: f64, t2075: f64, t2051: f64, t670: f64, t2097: f64, t3999: f64, t2055: f64, t4147: f64, t7535: f64, t2645: f64, t4366: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t28425 = t2718 * t2061;
    let t28472 = t198 * t2075;
    let t28658 = t2051 * t670;
    let t28911 = t3999 * t2097;
    let t28974 = t670 * t2055;
    let t33183 = t4147 * t7535;
    let t39588 = t4366 * t2645;
    (t28425, t28472, t28658, t28911, t28974, t33183, t39588)
}
