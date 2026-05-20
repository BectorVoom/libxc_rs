//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1214/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1214<F: Float>(t93058: F, t93063: F, t93067: F, t93069: F, t93073: F, t93075: F, t93077: F, t93080: F, t93084: F, t93086: F, t93088: F, t93091: F, t93093: F, t93095: F) -> F {
    let t93097 = -F::cast_from(0.76230004213927992339e-4_f64) * t93058 - F::cast_from(0.25724410870841842183e-2_f64) * t93063 - F::cast_from(0.13605355082800796533e0_f64) * t93067 + F::cast_from(0.24009450146119052704e-1_f64) * t93069 + F::cast_from(0.32524801797942610064e-2_f64) * t93073 - F::cast_from(0.17149607247227894789e-2_f64) * t93075 - F::cast_from(0.30492001685571196935e-3_f64) * t93077 + F::cast_from(0.42874018118069736972e-4_f64) * t93080 - F::cast_from(0.85748036236139473944e-4_f64) * t93084 - F::cast_from(0.12004725073059526352e0_f64) * t93086 - F::cast_from(0.45732285992607719437e-3_f64) * t93088 + F::cast_from(0.42874018118069736972e-4_f64) * t93091 - F::cast_from(0.51448821741683684367e-1_f64) * t93093 + F::cast_from(0.15246000842785598468e-2_f64) * t93095;
    t93097
}
