//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1150/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1150(t119842: f64, t2453: f64, t25301: f64, t32477: f64, t121803: f64, t1955: f64, t119869: f64, t25331: f64, t32478: f64, t2470: f64, t32470: f64, t32474: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t121855 = 0.98339826130601561944e-2_f64 * t119842;
    let t121869 = 0.3427046870806409921e-2_f64 * t2453 * t32477 * t25301;
    let t121870 = t1955 * t121803;
    let t121879 = 0.35702867204846465857e-4_f64 * t119869;
    let t121881 = 0.19274729307122665472e-1_f64 * t32478 * t25331;
    let t121884 = t32470 * t2470;
    let t121886 = 0.33852964522850660984e-1_f64 * t32474 * t121884;
    (t121855, t121869, t121870, t121879, t121881, t121884, t121886)
}
