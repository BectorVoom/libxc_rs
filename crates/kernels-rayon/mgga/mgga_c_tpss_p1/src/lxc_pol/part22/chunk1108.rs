//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1108/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1108(t1543: f64, t2993: f64, t2975: f64, t4184: f64, t3001: f64, t4180: f64, t1080: f64, t12177: f64, t12180: f64, t12183: f64, t12187: f64, t12190: f64, t12194: f64, t12201: f64, t2930: f64, t2955: f64, t2974: f64, t2999: f64, t4163: f64, t4185: f64, t9359: f64, t9370: f64, t9373: f64, t9424: f64, t9465: f64) -> f64 {
    let t12204 = t1543 * t2993;
    let t12207 = t4184 * t2975;
    let t12210 = t4180 * t3001;
    let t12211 = t12210 * t1080;
    let t12214 = t4184 * t2993;
    let t12217 = -4.0_f64 * t2930 * t12177 - 2.0_f64 * t2930 * t12180 - 0.19298375398431042081e3_f64 * t9424 * t12183 + 0.64327917994770140268e2_f64 * t2955 * t12187 + 0.32163958997385070134e2_f64 * t2955 * t12190 + 0.2069040516770936012e4_f64 * t9465 * t12194 - 0.23392894490538584828e1_f64 * t9359 * t4163 + 0.34631718211362927518e2_f64 * t9370 * t4185 - 0.23392894490538584828e1_f64 * t2974 * t12201 - 0.11696447245269292414e1_f64 * t2974 * t12204 - 0.10389515463408878255e3_f64 * t9373 * t12207 + 0.34631718211362927518e2_f64 * t2999 * t12211 + 0.17315859105681463759e2_f64 * t2999 * t12214;
    t12217
}
