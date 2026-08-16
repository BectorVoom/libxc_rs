//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1305/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1305(t106024: f64, t106030: f64, t106033: f64, t106037: f64, t106040: f64, t106042: f64, t106048: f64, t106050: f64, t92989: f64, t98976: f64, t98979: f64, t99002: f64, t99009: f64, t99013: f64) -> f64 {
    let t113206 = -0.12004725073059526352e0_f64 * t106024 - 0.1084295579938911763e-3_f64 * t98976 + 0.15246000842785598468e-4_f64 * t98979 + 0.81312004494856525162e-3_f64 * t99002 - t92989 - 0.13605355082800796533e0_f64 * t99009 - 0.85748036236139473944e-4_f64 * t106030 + 0.42874018118069736972e-4_f64 * t106033 + 0.32524801797942610064e-2_f64 * t99013 - 0.30492001685571196935e-3_f64 * t106037 + 0.42874018118069736972e-4_f64 * t106040 + 0.60023625365297631762e-2_f64 * t106042 - 0.76230004213927992339e-4_f64 * t106048 + 0.15246000842785598468e-3_f64 * t106050;
    t113206
}
