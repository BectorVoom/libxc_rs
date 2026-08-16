//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1132/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1132(t2026: f64, t3640: f64, t5939: f64, t154: f64, t18086: f64, t276: f64, t3542: f64, t735: f64, t9546: f64, t9583: f64, t3515: f64, t5688: f64) -> (f64, f64, f64, f64, f64) {
    let t25189 = t2026 * t5939 * t3640;
    let t25198 = t276 * t154 * t18086 * t3542;
    let t25207 = t735 * t9546;
    let t25212 = t735 * t9583;
    let t25218 = t276 * t154 * t5688 * t3515;
    (t25189, t25198, t25207, t25212, t25218)
}
