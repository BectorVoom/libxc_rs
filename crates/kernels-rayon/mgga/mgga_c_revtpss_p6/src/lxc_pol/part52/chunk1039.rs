//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1039/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1039(t31747: f64, t31750: f64, t31763: f64, t2061: f64, t7048: f64, t8650: f64, t31812: f64, t8651: f64, t886: f64, t1955: f64, t7398: f64, t31828: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t32437 = 0.37645955677973955999e-4_f64 * t31747;
    let t32438 = 0.66934509195437693771e-4_f64 * t31750;
    let t32439 = 0.263521689745817692e-2_f64 * t31763;
    let t32440 = t2061 * t7048;
    let t32441 = t8650 * t32440;
    let t32445 = t31812 * t8651 * t886;
    let t32450 = t1955 * t7398;
    let t32456 = 0.3718732920905101082e-4_f64 * t31828;
    (t32437, t32438, t32439, t32440, t32441, t32445, t32450, t32456)
}
