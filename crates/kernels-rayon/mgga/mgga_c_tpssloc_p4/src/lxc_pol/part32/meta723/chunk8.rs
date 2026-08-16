//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2314/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2314(t24574: f64, t29790: f64, t29763: f64, t8067: f64, t94490: f64, t11914: f64, t1201: f64, t1244: f64, t1246: f64, t18572: f64, t18940: f64, t19153: f64, t2144: f64, t2152: f64, t27406: f64, t27460: f64, t27466: f64, t27474: f64, t27478: f64, t29708: f64, t29773: f64, t4733: f64, t5011: f64, t5064: f64, t7283: f64, t7362: f64, t8054: f64, t95726: f64) -> f64 {
    let t103950 = t24574 * t29790;
    let t103954 = t24574 * t29763;
    let t103959 = t94490 * t8067;
    let t103978 = -0.54831135561607547883e-2_f64 * t103950 + t11914 * t29708 * t19153 - 0.18277045187202515961e-2_f64 * t103954 + 0.14621636149762012769e-1_f64 * t27406 * t27466 + t18572 * t2152 + 0.48738787165873375897e-2_f64 * t103959 - 0.54831135561607547884e-2_f64 * t7283 * t7362 * t27460 * t4733 - 0.36554090374405031923e-2_f64 * t95726 + 2.0_f64 * t1244 * t8054 * t5011 * t1246 + t1201 * t29773 + 0.14621636149762012769e-1_f64 * t27406 * t27474 + 2.0_f64 * t5064 * t27478 + t1244 * t2144 * t18940 * t1246;
    t103978
}
