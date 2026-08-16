//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 200/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk200(t782: f64, t788: f64, t722: f64, t772: f64, t737: f64, t749: f64, t240: f64, t753: f64, t567: f64, t157: f64, t32: f64, t5: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t791 = 1.0_f64 + 0.2698618307426597582e-1_f64 * t782 * t788;
    let t792 = f64::ln(t791);
    let t794 = 1.0_f64 + 0.193e0_f64 * t792;
    let t795 = 1.0_f64 / t794;
    let t798 = t772 * t795 + 0.17411041666666666666e-2_f64 * t722;
    let t801 = 1.0_f64 + 0.9375e-1_f64 * t737 - 0.101171875e-1_f64 * t749;
    let t802 = 1.0_f64 / t801;
    let t806 = t753 + t240 * (t798 * t802 - t753);
    let t807 = t567 * t806;
    let t812 = 0.11073577833333333333e-2_f64 * t5 * t157 * t32;
    (t791, t794, t795, t798, t801, t802, t806, t807, t812)
}
