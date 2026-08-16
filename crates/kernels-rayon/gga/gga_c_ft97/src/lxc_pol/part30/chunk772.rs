//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 772/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk772(t33771: f64, t684: f64, t10079: f64, t24793: f64, t6162: f64, t242: f64, t33596: f64, t1424: f64, t6187: f64, t729: f64, t762: f64, t1901: f64, t33743: f64, t33747: f64, t33748: f64, t33751: f64, t33756: f64, t33761: f64, t33765: f64, t33768: f64, t446: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t33772 = t33771 * t684;
    let t33773 = t10079 * t33772;
    let t33776 = t24793 * t6162;
    let t33779 = t242 * t33596;
    let t33782 = t1424 * t6187;
    let t33784 = t729 * t762 * t33782;
    let t33787 = -t446 * t33743 / 3.0_f64 + t33747 - t446 * t33748 / 3.0_f64 - 2.0_f64 / 3.0_f64 * t446 * t33751 + t1901 * t33756 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t1901 * t33761 - t33765 + t446 * t33768 / 3.0_f64 - 2.0_f64 / 9.0_f64 * t1901 * t33773 + 2.0_f64 / 9.0_f64 * t1901 * t33776 + 2.0_f64 / 3.0_f64 * t446 * t33779 + 2.0_f64 / 3.0_f64 * t446 * t33784;
    (t33772, t33773, t33776, t33779, t33782, t33784, t33787)
}
