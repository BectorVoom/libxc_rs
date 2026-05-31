//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1181/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1181<F: Float>(t34836: F, t34839: F, t34844: F, t34851: F, t34853: F, t34855: F, t34865: F, t30769: F, t30773: F, t30775: F, t30777: F, t34841: F, t34849: F, t34857: F, t34859: F, t34862: F, t34869: F, t34873: F) -> F {
    let t37267 = F::cast_from(0.85748036236139473944e-3_f64) * t34836;
    let t37268 = F::cast_from(0.12579236915841660828e-2_f64) * t34839;
    let t37271 = F::cast_from(0.34299214494455789578e-2_f64) * t34844;
    let t37276 = F::cast_from(0.16006300097412701803e-1_f64) * t34851;
    let t37277 = F::cast_from(0.16006300097412701803e-1_f64) * t34853;
    let t37278 = F::cast_from(0.80031500487063509014e-2_f64) * t34855;
    let t37282 = F::cast_from(0.28582678745379824648e-3_f64) * t34865;
    let t37285 = -t37267 + t37268 - F::cast_from(0.20579528696673473747e-1_f64) * t34841 + F::cast_from(0.13719685797782315831e-1_f64) * t30769 + t37271 + F::cast_from(0.85748036236139473944e-3_f64) * t30773 - F::cast_from(0.34299214494455789578e-2_f64) * t30775 + F::cast_from(0.34299214494455789578e-2_f64) * t30777 - F::cast_from(0.11321313224257494745e-1_f64) * t34849 + t37276 - t37277 + t37278 + F::cast_from(0.34299214494455789578e-2_f64) * t34857 + F::cast_from(0.17149607247227894789e-2_f64) * t34859 - F::cast_from(0.42874018118069736972e-3_f64) * t34862 - t37282 - F::cast_from(0.916875e-1_f64) * t34869 - F::cast_from(0.916875e-1_f64) * t34873;
    t37285
}
