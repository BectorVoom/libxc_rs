//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1109/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1109(t121030: f64, t1426: f64, t786: f64, t26050: f64, t7063: f64, t2470: f64, t32219: f64, t32223: f64, t1419: f64, t31805: f64, t1381: f64, t8590: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t121089 = t121030 * t1426;
    let t121090 = t786 * t121089;
    let t121091 = t121090 * t26050;
    let t121093 = t7063 * t121089;
    let t121094 = t121093 * t26050;
    let t121096 = t32219 * t2470;
    let t121098 = 0.34270468708064099208e-1_f64 * t32223 * t121096;
    let t121099 = t31805 * t1419;
    let t121101 = t121099 * t8590 * t1381;
    (t121090, t121091, t121093, t121094, t121096, t121098, t121101)
}
