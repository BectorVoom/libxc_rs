//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1449/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1449(t1269: f64, t1287: f64, t5284: f64, t17633: f64, t5458: f64, t17482: f64, t3769: f64, t3783: f64, t12713: f64, t5332: f64, t13147: f64, t487: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17826 = t1269 * t5284 * t1287;
    let t17829 = t17633 * t5458;
    let t17834 = t17482 * t3769;
    let t17837 = t17482 * t3783;
    let t17840 = t5332 * t12713;
    let t17845 = t13147 * t487;
    (t17826, t17829, t17834, t17837, t17840, t17845)
}
