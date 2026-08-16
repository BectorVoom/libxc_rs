//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1152/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1152(t1444: f64, t4057: f64, t1426: f64, t94609: f64, t7063: f64, t25877: f64, t94801: f64, t1419: f64, t786: f64, t2453: f64, t25949: f64, t25898: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t94868 = t4057 * t1444;
    let t94878 = t94609 * t1426;
    let t94879 = t7063 * t94878;
    let t94886 = t94801 * t25877;
    let t94889 = t786 * t1419;
    let t94890 = t94889 * t25877;
    let t94894 = t786 * t94878;
    let t94913 = t2453 * t25949;
    let t94921 = t94889 * t25898;
    (t94868, t94879, t94886, t94890, t94894, t94913, t94921)
}
