//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1674/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1674(t77804: f64, t88085: f64, t88093: f64, t88104: f64, t88108: f64, t88114: f64, t88122: f64, t88130: f64, t88220: f64, t88222: f64, t88224: f64, t88226: f64, t88229: f64, t88232: f64) -> f64 {
    let t88412 = -0.379785e1_f64 * t88220 - 0.46074375e0_f64 * t88222 + 0.614325e0_f64 * t88224 + 0.85451625e1_f64 * t88226 - 0.21908444444444444444e0_f64 * t88229 + 0.65725333333333333332e0_f64 * t88232 + 0.71752e1_f64 * t88085 + 0.17938e1_f64 * t88093 - 0.88582716049382716048e0_f64 * t88104 - 0.29896666666666666667e0_f64 * t88108 + 0.39862222222222222223e1_f64 * t88114 - 0.71752000000000000002e1_f64 * t88122 - 0.59793333333333333333e0_f64 * t88130 - 0.13145066666666666666e1_f64 * t77804;
    t88412
}
