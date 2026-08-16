//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2594/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2594(t1227: f64, t13969: f64, t15544: f64, t15655: f64, t15636: f64, t3515: f64, t1174: f64, t44571: f64, t4724: f64, t11778: f64, t43791: f64, t11720: f64, t11722: f64, t11748: f64, t15498: f64, t3587: f64, t44725: f64, t44811: f64, t44863: f64, t45030: f64, t4582: f64, t48497: f64, t4889: f64, t4977: f64, t52575: f64) -> f64 {
    let t52580 = t1227 * t13969 * t15544;
    let t52583 = t1227 * t13969 * t15655;
    let t52586 = t3515 * t13969 * t15636;
    let t52599 = t1174 * t44571 * t4724;
    let t52600 = t52599 / 324.0_f64;
    let t52601 = t11778 * t43791;
    let t52606 = -5.0_f64 / 864.0_f64 * t15498 * t3587 + t52575 / 108.0_f64 - t4889 * t11748 / 27.0_f64 + 5.0_f64 / 6912.0_f64 * t52580 + 5.0_f64 / 1152.0_f64 * t52583 - t52586 / 768.0_f64 + t44863 * t4582 * t4977 * t44725 * t11720 / 128.0_f64 - 3.0_f64 / 256.0_f64 * t45030 * t4582 * t4977 * t11722 + t44811 / 432.0_f64 - t52600 - 5.0_f64 / 432.0_f64 * t1227 * t4582 * t52601 * t48497;
    t52606
}
