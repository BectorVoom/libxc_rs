//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1050/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1050<F: Float>(t34745: F, t34753: F, t34794: F, t34836: F, t34839: F, t34851: F, t34853: F, t34855: F, t34865: F, t34893: F, t34895: F, t34957: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t37230 = F::new(0.34299214494455789578e-2) * t34745;
    let t37234 = F::new(0.64025200389650807212e-1) * t34753;
    let t37249 = F::new(0.31448092289604152068e-2) * t34794;
    let t37267 = F::new(0.85748036236139473944e-3) * t34836;
    let t37268 = F::new(0.12579236915841660828e-2) * t34839;
    let t37276 = F::new(0.16006300097412701803e-1) * t34851;
    let t37277 = F::new(0.16006300097412701803e-1) * t34853;
    let t37278 = F::new(0.80031500487063509014e-2) * t34855;
    let t37282 = F::new(0.28582678745379824648e-3) * t34865;
    let t37291 = F::new(0.3361875e0) * t34893;
    let t37292 = F::new(0.3361875e0) * t34895;
    let t37311 = F::new(0.57165357490759649296e-3) * t34957;
    (t37230, t37234, t37249, t37267, t37268, t37276, t37277, t37278, t37282, t37291, t37292, t37311)
}
