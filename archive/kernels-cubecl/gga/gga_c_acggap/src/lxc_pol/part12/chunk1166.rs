//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1166/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1166<F: Float>(t34221: F, t34237: F, t34239: F, t34255: F, t34271: F, t34223: F, t34225: F, t34227: F, t34229: F, t34233: F, t34242: F, t34246: F, t34251: F, t34259: F, t34263: F, t34265: F, t34269: F, t34273: F) -> F {
    let t36987 = F::cast_from(0.12579236915841660828e-2_f64) * t34221;
    let t36993 = F::cast_from(0.42874018118069736972e-3_f64) * t34237;
    let t36994 = F::cast_from(0.34299214494455789578e-2_f64) * t34239;
    let t36998 = F::cast_from(0.85748036236139473944e-3_f64) * t34255;
    let t37003 = F::cast_from(0.17149607247227894789e-2_f64) * t34271;
    let t37005 = -t36987 + F::cast_from(0.13719685797782315831e-1_f64) * t34223 - F::cast_from(0.68598428988911579156e-2_f64) * t34225 + F::cast_from(0.10289764348336736873e-1_f64) * t34227 + F::cast_from(0.51448821741683684367e-2_f64) * t34229 - F::cast_from(0.37737710747524982483e-2_f64) * t34233 + t36993 - t36994 + F::cast_from(0.62896184579208304138e-3_f64) * t34242 + F::cast_from(0.18868855373762491241e-2_f64) * t34246 + F::cast_from(0.18868855373762491242e-1_f64) * t34251 - t36998 + F::cast_from(0.18868855373762491241e-1_f64) * t34259 - F::cast_from(0.12579236915841660828e-2_f64) * t34263 + F::cast_from(0.17149607247227894789e-2_f64) * t34265 - F::cast_from(0.75475421495049964964e-2_f64) * t34269 - t37003 - F::cast_from(0.80031500487063509014e-2_f64) * t34273;
    t37005
}
