//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 933/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk933<F: Float>(t34620: F, t34626: F, t34632: F, t34659: F, t34702: F, t34704: F, t34710: F, t34712: F, t34745: F, t34753: F, t34794: F, t34836: F, t34839: F, t34851: F, t34853: F, t34855: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t37180 = 0.37737710747524982482e-2 * t34620;
    let t37182 = 0.18868855373762491241e-2 * t34626;
    let t37184 = 0.37737710747524982482e-1 * t34632;
    let t37197 = 7.0 / 36.0 * t34659;
    let t37213 = 0.25724410870841842184e-1 * t34702;
    let t37214 = 0.1543464652250510531e-1 * t34704;
    let t37216 = 0.25724410870841842184e-2 * t34710;
    let t37217 = 0.25724410870841842184e-2 * t34712;
    let t37230 = 0.34299214494455789578e-2 * t34745;
    let t37234 = 0.64025200389650807212e-1 * t34753;
    let t37249 = 0.31448092289604152068e-2 * t34794;
    let t37267 = 0.85748036236139473944e-3 * t34836;
    let t37268 = 0.12579236915841660828e-2 * t34839;
    let t37276 = 0.16006300097412701803e-1 * t34851;
    let t37277 = 0.16006300097412701803e-1 * t34853;
    let t37278 = 0.80031500487063509014e-2 * t34855;
    (t37180, t37182, t37184, t37197, t37213, t37214, t37216, t37217, t37230, t37234, t37249, t37267, t37268, t37276, t37277, t37278)
}
