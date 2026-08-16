//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2510/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2510(t10886: f64, t14833: f64, t808: f64, t241: f64, t40322: f64, t820: f64, t2659: f64, t2783: f64, t816: f64, t853: f64, t14688: f64, t40731: f64) -> (f64, f64, f64, f64, f64) {
    let t50706 = t10886 * t808 * t14833;
    let t50707 = 0.15246000842785598468e-3_f64 * t50706;
    let t50757 = t820 * t40322 * t241;
    let t50768 = t816 * t2659 * t2783;
    let t50769 = t808 * t853;
    let t50773 = t40731 * t14688;
    (t50707, t50757, t50768, t50769, t50773)
}
