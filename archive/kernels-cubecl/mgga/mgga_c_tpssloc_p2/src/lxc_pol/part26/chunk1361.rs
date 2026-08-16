//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1361/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1361<F: Float>(t24807: F, t24826: F, t225: F, t24705: F, t24574: F, t24860: F, t24594: F, t24847: F, t974: F, t27551: F, t7327: F, t11154: F, t1201: F, t1215: F, t1244: F, t1246: F, t24589: F, t24757: F, t24776: F, t24792: F, t24804: F, t24812: F, t24858: F, t27489: F, t3252: F, t3565: F, t3604: F, t4978: F, t7283: F, t7362: F, t7363: F, t7364: F, t7389: F, t85827: F, t86016: F) -> F {
    let t86057 = t24826 * t24807;
    let t86059 = t24705 * t225;
    let t86073 = t24574 * t24860;
    let t86076 = t24847 * t974 * t24594;
    let t86077 = t7327 * t27551;
    let t86089 = F::cast_from(0.49348022005446793095e-1_f64) * t24812 * t27489 * t85827 * t4978 + F::cast_from(0.82246703342411321826e-2_f64) * t86057 + F::cast_from(0.82246703342411321826e-2_f64) * t24589 * t86059 * t7364 + F::cast_from(3.0_f64) * t1201 * t24792 + F::cast_from(3.0_f64) * t1244 * t24757 * t1215 * t1246 - F::cast_from(0.82246703342411321826e-2_f64) * t7283 * t7362 * t24858 * t3252 - F::cast_from(0.54831135561607547883e-2_f64) * t86073 + F::cast_from(0.10966227112321509577e-1_f64) * t86076 * t86077 * t86016 + F::cast_from(0.21932454224643019154e-1_f64) * t7283 * t24776 * t7363 * t11154 + F::cast_from(3.0_f64) * t3565 * t7389 + F::cast_from(3.0_f64) * t3604 * t24804;
    t86089
}
