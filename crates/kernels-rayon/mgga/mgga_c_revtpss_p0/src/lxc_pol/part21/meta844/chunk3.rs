//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3160/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3160(t12486: f64, t1756: f64, t12485: f64, t1749: f64, t12423: f64, t12476: f64, t12481: f64, t12487: f64, t12488: f64, t12501: f64, t12514: f64, t12553: f64, t16971: f64, t16974: f64, t17032: f64, t1757: f64, t3497: f64, t3521: f64, t45163: f64, t5181: f64, t5184: f64, t57831: f64, t57833: f64, t57835: f64, t57837: f64, t57840: f64, t57856: f64) -> f64 {
    let t58259 = t12486 * t1756;
    let t58262 = t1749 * t12485;
    let t58275 = 0.5848223622634646207e0_f64 * t45163 * t1757 + 0.17544670867903938621e1_f64 * t12476 * t5181 + 6.0_f64 * t17032 * t12514 + t57831 + t57833 - t57835 - t57837 + t57840 - 0.31168546390226634766e3_f64 * t58259 * t12501 - 0.10389515463408878255e3_f64 * t58262 * t12488 - t57856 + 0.10526802520742363173e2_f64 * t12481 * t16971 + 0.10526802520742363173e2_f64 * t3521 * t5181 * t3497 + 0.6233709278045326953e3_f64 * t12553 * t5184 * t12487 + 18.0_f64 * t12423 * t16974;
    t58275
}
