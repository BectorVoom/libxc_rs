//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1050/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1050(t34745: f64, t34753: f64, t34794: f64, t34836: f64, t34839: f64, t34851: f64, t34853: f64, t34855: f64, t34865: f64, t34893: f64, t34895: f64, t34957: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t37230 = 0.34299214494455789578e-2_f64 * t34745;
    let t37234 = 0.64025200389650807212e-1_f64 * t34753;
    let t37249 = 0.31448092289604152068e-2_f64 * t34794;
    let t37267 = 0.85748036236139473944e-3_f64 * t34836;
    let t37268 = 0.12579236915841660828e-2_f64 * t34839;
    let t37276 = 0.16006300097412701803e-1_f64 * t34851;
    let t37277 = 0.16006300097412701803e-1_f64 * t34853;
    let t37278 = 0.80031500487063509014e-2_f64 * t34855;
    let t37282 = 0.28582678745379824648e-3_f64 * t34865;
    let t37291 = 0.3361875e0_f64 * t34893;
    let t37292 = 0.3361875e0_f64 * t34895;
    let t37311 = 0.57165357490759649296e-3_f64 * t34957;
    (t37230, t37234, t37249, t37267, t37268, t37276, t37277, t37278, t37282, t37291, t37292, t37311)
}
