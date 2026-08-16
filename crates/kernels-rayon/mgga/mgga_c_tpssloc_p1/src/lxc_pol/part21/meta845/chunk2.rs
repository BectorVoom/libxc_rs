//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3058/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3058(t3265: f64, t3313: f64, t6021: f64, t11190: f64, t5989: f64, t14850: f64, t14937: f64, t3375: f64, t6063: f64, t1136: f64, t11365: f64, t14829: f64, t15153: f64, t15165: f64, t15179: f64, t15219: f64, t1695: f64, t18615: f64, t18622: f64, t3376: f64, t3377: f64, t3378: f64, t3395: f64, t3401: f64, t3403: f64, t436: f64, t44155: f64, t51382: f64, t51389: f64, t51392: f64, t51486: f64, t51521: f64, t51727: f64, t6085: f64, t6088: f64, t63280: f64, t63283: f64, t63290: f64, t63325: f64, t63346: f64, t63376: f64, t63424: f64) -> (f64, f64, f64, f64) {
    let t63446 = 6.0_f64 * t3313 * t6021 * t3265;
    let t63449 = 24.0_f64 * t11190 * t5989 * t3265;
    let t63451 = 12.0_f64 * t14850 * t14937;
    let t63454 = t6063 * t3375;
    let t63457 = 0.8276162067083744048e4_f64 * t51486 * t51521 * t1136 + 24.0_f64 * t51382 * t15153 - 0.4155806185363551302e3_f64 * t51727 * t15219 - t63280 + 0.14035736694323150897e2_f64 * t51389 * t15179 + 0.34631718211362927518e2_f64 * t3401 * t63283 * t3403 + t63290 - 0.310907e-1_f64 * (t63325 + t63346 + t63376 + t63424) * t436 - 0.23392894490538584828e1_f64 * t3376 * t1695 * t14829 - 0.10389515463408878255e3_f64 * t11365 * t6088 * t3395 - 0.12304822629859687989e5_f64 * t44155 * t18622 * t3377 - 0.11696447245269292414e1_f64 * t3376 * t6085 * t3395 - 0.10389515463408878255e3_f64 * t11365 * t18615 * t3377 - t63446 + t63449 - t63451 - 0.77193501593724168323e3_f64 * t51392 * t15165 - 0.11696447245269292414e1_f64 * t63454 * t3378;
    (t63446, t63449, t63451, t63457)
}
