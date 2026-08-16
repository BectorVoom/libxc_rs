//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1086/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1086(t32608: f64, t32628: f64, t3: f64, t112: f64, t8919: f64, t31277: f64, t31279: f64, t31282: f64, t31284: f64, t31287: f64, t31940: f64, t31942: f64, t31944: f64, t577: f64, t671: f64, t8508: f64) -> (f64, f64, f64, f64) {
    let t32629 = t32608 + t32628;
    let t32630 = t3 * t32629;
    let t32643 = t8919 * t112;
    let t32649 = 0.45e1_f64 * t32629 * t577 + 0.135e2_f64 * t32643 * t671 + 27.0_f64 * t31940 + 54.0_f64 * t31942 + 27.0_f64 * t31944 + t31277 + t31279 + t31282 + t31284 + t31287 + t8508;
    (t32629, t32630, t32643, t32649)
}
