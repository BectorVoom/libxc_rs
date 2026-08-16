//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3267/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3267(t18414: f64, t40799: f64, t9794: f64, t10760: f64, t18418: f64, t18392: f64, t236: f64, t807: f64, t854: f64, t18643: f64, t40731: f64, t10779: f64, t10786: f64, t14931: f64, t61956: f64) -> (f64, f64, f64, f64, f64) {
    let t62012 = t40799 * t9794 * t18414;
    let t62015 = t10760 * t9794 * t18418;
    let t62021 = t807 * t236 * t854 * t18392;
    let t62029 = t40731 * t18643;
    let t62033 = t14931 * t10779 * t61956 * t10786;
    (t62012, t62015, t62021, t62029, t62033)
}
