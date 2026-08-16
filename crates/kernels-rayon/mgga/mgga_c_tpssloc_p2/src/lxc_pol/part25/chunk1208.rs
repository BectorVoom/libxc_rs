//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1208/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1208(t84873: f64, t84894: f64, t84916: f64, t84937: f64, t24234: f64, t814: f64, t10016: f64, t2051: f64, t226: f64, t235: f64, t4291: f64, t812: f64, t81563: f64, t81568: f64, t81571: f64, t81575: f64, t81585: f64, t81589: f64, t81592: f64, t81595: f64, t81600: f64, t81602: f64, t81606: f64, t81610: f64, t81615: f64, t829: f64, t84842: f64, t84851: f64) -> (f64, f64) {
    let t84939 = t84873 + t84894 + t84916 + t84937;
    let t84945 = t814 * t24234;
    let t84949 = -0.19739208802178717238e0_f64 * t81563 + 0.9869604401089358619e-1_f64 * t81568 - 0.24674011002723396548e-1_f64 * t81571 + 0.9869604401089358619e-1_f64 * t81575 - 3.0_f64 * t4291 * t84842 * t829 + t10016 * t2051 - 0.29608813203268075857e0_f64 * t81585 + 0.9869604401089358619e-1_f64 * t81589 - 0.46058153871750340221e0_f64 * t81592 - 0.49348022005446793095e-1_f64 * t81595 - t84851 + 0.15626873635058151147e0_f64 * t81600 + 0.38381794893125283518e0_f64 * t81602 + t226 * t235 * t84939 + 0.19739208802178717238e0_f64 * t81606 + 0.9869604401089358619e-1_f64 * t81610 + 0.49348022005446793095e-1_f64 * t81615 - 3.0_f64 * t812 * t84945 * t829;
    (t84939, t84949)
}
