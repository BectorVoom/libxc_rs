//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2152/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2152(t19696: f64, t7121: f64, t20016: f64, t25500: f64, t19463: f64, t1972: f64, t100030: f64, t100302: f64, t100345: f64, t1028: f64, t1665: f64, t19770: f64, t19940: f64, t19993: f64, t19998: f64, t25490: f64, t25522: f64, t27479: f64, t4854: f64, t6278: f64, t6339: f64, t7117: f64, t93720: f64, t93728: f64) -> (f64, f64) {
    let t107048 = t19696 * t7121;
    let t107064 = t25500 * t20016;
    let t107072 = t19463 * t1972;
    let t107082 = -0.57165357490759649296e-3_f64 * t25522 * t19940 - 0.45732285992607719437e-2_f64 * t93728 * t6339 + 0.57165357490759649296e-3_f64 * t107064 - 0.85748036236139473944e-3_f64 * t100345 * t1665 - 0.85748036236139473944e-3_f64 * t27479 * t4854 - 0.42874018118069736972e-3_f64 * t25490 * t6278 - 0.42874018118069736972e-3_f64 * t107072 * t1028 - 0.42874018118069736972e-3_f64 * t7117 * t19770 + 0.95275595817932748827e-4_f64 * t93720 - 0.11433071498151929859e-2_f64 * t100030 * t19993 + 0.11433071498151929859e-2_f64 * t100302 * t19998;
    (t107048, t107082)
}
