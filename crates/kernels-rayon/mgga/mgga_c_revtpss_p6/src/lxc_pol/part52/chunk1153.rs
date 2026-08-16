//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1153/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1153(t32275: f64, t32707: f64, t94801: f64, t122295: f64, t94390: f64, t28911: f64, t8584: f64, t25875: f64, t25901: f64, t32268: f64, t2470: f64, t32706: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t122299 = t94801 * t32275 * t32707;
    let t122309 = 0.50779446784275991476e-2_f64 * t94390 * t32275 * t122295;
    let t122310 = t8584 * t28911;
    let t122311 = t25875 * t122310;
    let t122312 = t122311 * t25901;
    let t122314 = t32268 * t122310;
    let t122315 = t122314 * t25901;
    let t122317 = t32706 * t2470;
    (t122299, t122309, t122311, t122312, t122314, t122315, t122317)
}
