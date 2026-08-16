//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1023/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1023(t12659: f64, t333: f64, t335: f64, t337: f64, t339: f64, t341: f64, t1083: f64, t1085: f64, t1087: f64, t1089: f64, t12657: f64, t2958: f64, t343: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12660 = t333 * t12659;
    let t12662 = t335 * t12659;
    let t12664 = t337 * t12659;
    let t12666 = t339 * t12659;
    let t12668 = t341 * t12659;
    let t12681 = -0.17408e1_f64 * t12657 - 0.8704e0_f64 * t12660 - 0.4607056813647e1_f64 * t12662 + 0.122462410087e2_f64 * t12664 - 0.957855118103e1_f64 * t12666 + 0.3101306810232e1_f64 * t12668 - 0.362942158544e0_f64 * t343 * t12659 - 0.64e0_f64 * t12659 + 0.734774460522e2_f64 * t1083 * t2958 - 0.11494261417236e3_f64 * t1085 * t2958 + 0.6202613620464e2_f64 * t1087 * t2958 - 0.1088826475632e2_f64 * t1089 * t2958;
    (t12660, t12662, t12664, t12666, t12668, t12681)
}
