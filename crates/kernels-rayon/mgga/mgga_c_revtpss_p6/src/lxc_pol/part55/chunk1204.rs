//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1204/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1204(t121818: f64, t121820: f64, t121825: f64, t121827: f64, t121830: f64, t121836: f64, t121838: f64, t121840: f64, t126099: f64, t32430: f64, t32441: f64, t34075: f64) -> f64 {
    let t127640 = -0.56468933516960933999e-3_f64 * t126099 + 0.42839803248826764462e-1_f64 * t121818 - 0.25702851531048074406e-1_f64 * t121820 - t121825 - 0.14279934416275588154e-1_f64 * t121827 + 0.57119737665102352616e0_f64 * t34075 * t32430 + 0.57119737665102352616e0_f64 * t34075 * t32441 + 0.14456046980341999104e-1_f64 * t121830 - t121836 + 0.25389723392137995738e-1_f64 * t121838 - t121840;
    t127640
}
