//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1230/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1230(t10110: f64, t105258: f64, t105267: f64, t105423: f64, t105428: f64, t17052: f64, t21054: f64, t29080: f64, t4268: f64, t5636: f64, t7087: f64, t7830: f64, t7841: f64, t855: f64, t86955: f64, t98213: f64, t98237: f64) -> f64 {
    let t108361 = -0.19739208802178717238e0_f64 * t105258 + 12.0_f64 * t4268 * t29080 - 0.9869604401089358619e-1_f64 * t98213 - 18.0_f64 * t855 * t10110 * t7841 * t5636 - 0.49348022005446793095e-1_f64 * t105267 - 0.14804406601634037928e0_f64 * t98237 + 6.0_f64 * t7087 * t21054 + 0.38381794893125283518e0_f64 * t86955 - 0.9869604401089358619e-1_f64 * t105423 + 6.0_f64 * t17052 * t7830 + 0.16449340668482264365e-1_f64 * t105428;
    t108361
}
