//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1015/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1015(t32183: f64, t32186: f64, t786: f64, t7063: f64, t1385: f64, t239: f64) -> (f64, f64, f64) {
    let t32187 = t786 * t32183 * t32186;
    let t32188 = 0.18822977838986977999e-4_f64 * t32187;
    let t32190 = t7063 * t32183 * t32186;
    let t32191 = 0.33467254597718846885e-4_f64 * t32190;
    let t32192 = t1385 * t239;
    (t32188, t32191, t32192)
}
