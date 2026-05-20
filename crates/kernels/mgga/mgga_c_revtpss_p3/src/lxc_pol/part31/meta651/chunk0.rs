//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2152/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2152<F: Float>(t19696: F, t7121: F, t20016: F, t25500: F, t19463: F, t1972: F, t100030: F, t100302: F, t100345: F, t1028: F, t1665: F, t19770: F, t19940: F, t19993: F, t19998: F, t25490: F, t25522: F, t27479: F, t4854: F, t6278: F, t6339: F, t7117: F, t93720: F, t93728: F) -> (F, F) {
    let t107048 = t19696 * t7121;
    let t107064 = t25500 * t20016;
    let t107072 = t19463 * t1972;
    let t107082 = -F::cast_from(0.57165357490759649296e-3_f64) * t25522 * t19940 - F::cast_from(0.45732285992607719437e-2_f64) * t93728 * t6339 + F::cast_from(0.57165357490759649296e-3_f64) * t107064 - F::cast_from(0.85748036236139473944e-3_f64) * t100345 * t1665 - F::cast_from(0.85748036236139473944e-3_f64) * t27479 * t4854 - F::cast_from(0.42874018118069736972e-3_f64) * t25490 * t6278 - F::cast_from(0.42874018118069736972e-3_f64) * t107072 * t1028 - F::cast_from(0.42874018118069736972e-3_f64) * t7117 * t19770 + F::cast_from(0.95275595817932748827e-4_f64) * t93720 - F::cast_from(0.11433071498151929859e-2_f64) * t100030 * t19993 + F::cast_from(0.11433071498151929859e-2_f64) * t100302 * t19998;
    (t107048, t107082)
}
