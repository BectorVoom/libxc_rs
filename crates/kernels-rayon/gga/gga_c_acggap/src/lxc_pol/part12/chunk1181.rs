//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1181/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1181(t34836: f64, t34839: f64, t34844: f64, t34851: f64, t34853: f64, t34855: f64, t34865: f64, t30769: f64, t30773: f64, t30775: f64, t30777: f64, t34841: f64, t34849: f64, t34857: f64, t34859: f64, t34862: f64, t34869: f64, t34873: f64) -> f64 {
    let t37267 = 0.85748036236139473944e-3_f64 * t34836;
    let t37268 = 0.12579236915841660828e-2_f64 * t34839;
    let t37271 = 0.34299214494455789578e-2_f64 * t34844;
    let t37276 = 0.16006300097412701803e-1_f64 * t34851;
    let t37277 = 0.16006300097412701803e-1_f64 * t34853;
    let t37278 = 0.80031500487063509014e-2_f64 * t34855;
    let t37282 = 0.28582678745379824648e-3_f64 * t34865;
    let t37285 = -t37267 + t37268 - 0.20579528696673473747e-1_f64 * t34841 + 0.13719685797782315831e-1_f64 * t30769 + t37271 + 0.85748036236139473944e-3_f64 * t30773 - 0.34299214494455789578e-2_f64 * t30775 + 0.34299214494455789578e-2_f64 * t30777 - 0.11321313224257494745e-1_f64 * t34849 + t37276 - t37277 + t37278 + 0.34299214494455789578e-2_f64 * t34857 + 0.17149607247227894789e-2_f64 * t34859 - 0.42874018118069736972e-3_f64 * t34862 - t37282 - 0.916875e-1_f64 * t34869 - 0.916875e-1_f64 * t34873;
    t37285
}
