//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2089/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2089(t23437: f64, t3103: f64, t10472: f64, t10474: f64, t10478: f64, t23535: f64, t10948: f64, t23540: f64, t6753: f64, t10961: f64, t6754: f64, t3077: f64, t6764: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t83046 = t23437 * t3103;
    let t83054 = t10472 * t10474 * sigma0 * t10478;
    let t83058 = t10472 * t23535 * t10478;
    let t83061 = t10948 * t23540;
    let t83065 = t10472 * t6753 * t10478;
    let t83068 = t10961 * t6754;
    let t83071 = t3077 * t6764;
    (t83046, t83054, t83058, t83061, t83065, t83068, t83071)
}
