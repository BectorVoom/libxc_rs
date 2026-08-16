//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1156/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1156(t1955: f64, t32689: f64, t4075: f64, t121167: f64, t25304: f64, t25946: f64, t32715: f64, t10073: f64, t25938: f64, t122281: f64, t121202: f64, t122317: f64, t32710: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t122407 = t1955 * t32689 * t4075;
    let t122413 = 0.26773803678175077507e-4_f64 * t121167;
    let t122435 = 0.45699670022203476294e-2_f64 * t25304 * t32715 * t25946;
    let t122438 = 0.4818682326780666368e-3_f64 * t10073 * t32689 * t25938;
    let t122443 = t1955 * t122281;
    let t122451 = 0.14932895752263002547e-1_f64 * t121202;
    let t122454 = 0.33852964522850660984e-1_f64 * t32710 * t122317;
    (t122407, t122413, t122435, t122438, t122443, t122451, t122454)
}
