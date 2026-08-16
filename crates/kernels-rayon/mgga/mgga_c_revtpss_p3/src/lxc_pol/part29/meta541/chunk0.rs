//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1875/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1875(t93302: f64, t95854: f64, t25310: f64, t26544: f64, t7064: f64, t95575: f64, t2067: f64, t41117: f64, t26502: f64, t786: f64, t789: f64, t93314: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t95855 = t93302 * t95854;
    let t95857 = t25310 * t26544;
    let t95859 = t7064 * t95575;
    let t95862 = 0.81814717454467823679e-4_f64 * t41117 * t2067;
    let t95866 = t786 * t26502 * t789;
    let t95872 = t93314 * t95854;
    (t95855, t95857, t95859, t95862, t95866, t95872)
}
