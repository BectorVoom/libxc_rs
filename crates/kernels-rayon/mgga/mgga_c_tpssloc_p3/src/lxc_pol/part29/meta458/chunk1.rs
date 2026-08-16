//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1779/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1779(t13229: f64, t232: f64, t815: f64, t23097: f64, t1891: f64, t22813: f64, t22816: f64, t1895: f64, t794: f64, t1899: f64, t2693: f64, t281: f64, t6598: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t23098 = t13229 * t232;
    let t23099 = t815 * t23098;
    let t23100 = t23097 * t23099;
    let t23102 = t22813 * t1891;
    let t23103 = t23102 * t22816;
    let t23104 = t794 * t1895;
    let t23105 = t23103 * t23104;
    let t23106 = 0.16821981705891829522e-4_f64 * t23105;
    let t23107 = t1899 * t2693;
    let t23108 = 119.0_f64 / 6912.0_f64 * t23107;
    let t23109 = t6598 * t281;
    (t23098, t23099, t23100, t23102, t23104, t23106, t23108, t23109)
}
