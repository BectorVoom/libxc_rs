//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 582/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk582(t3831: f64, t7877: f64, t1354: f64, t7897: f64, t1398: f64, t7740: f64, t1375: f64, t7744: f64, t1349: f64, t1391: f64, t158: f64, t173: f64, t3844: f64, t3848: f64, t3851: f64, t3852: f64, t3858: f64, t5802: f64, t5804: f64, t7710: f64) -> (f64, f64, f64, f64, f64) {
    let t8108 = t3831 * t7877;
    let t8111 = t1354 * t7897;
    let t8123 = t1398 * t7740;
    let t8126 = t1375 * t7744;
    let t8129 = -t3844 - t3848 + t3851 - t3852 + t3858 + 0.11955719325063177623e-1_f64 * t1349 * t7710 - 0.5179538907796306876e-4_f64 * t1391 * t7710 - 0.23911438650126355246e-1_f64 * t5802 + 0.20718155631185227504e-3_f64 * t5804 - 0.10082625e-4_f64 * t173 * t8123 - 0.3513e-2_f64 * t158 * t8126;
    (t8108, t8111, t8123, t8126, t8129)
}
