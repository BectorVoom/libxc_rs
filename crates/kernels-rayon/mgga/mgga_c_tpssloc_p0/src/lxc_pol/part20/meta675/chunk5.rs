//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2549/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2549(t11275: f64, t1670: f64, t1117: f64, t43976: f64, t11285: f64, t4857: f64, t11129: f64, t11303: f64, t11310: f64, t11365: f64, t11399: f64, t11437: f64, t11441: f64, t1155: f64, t15133: f64, t15146: f64, t15153: f64, t15207: f64, t15218: f64, t15225: f64, t1694: f64, t1695: f64, t3376: f64, t3377: f64, t3395: f64, t3401: f64, t43692: f64, t44155: f64, t44223: f64, t4858: f64, t4861: f64) -> (f64, f64) {
    let t51638 = t11275 * t1670;
    let t51641 = 0.1551780387578202009e4_f64 * t51638 * t43976 * t1117;
    let t51651 = t4857 * t11285;
    let t51664 = -0.35089341735807877242e1_f64 * t3376 * t15133 * t1155 - 0.35089341735807877242e1_f64 * t3376 * t4858 * t3395 - 0.31168546390226634765e3_f64 * t11365 * t15218 * t3377 - 0.11696447245269292414e1_f64 * t3376 * t1695 * t11399 - 0.12304822629859687989e5_f64 * t44155 * t15225 * t11129 - 6.0_f64 * t15207 * t11437 - 12.0_f64 * t11303 * t15153 - t51641 - 0.14035736694323150897e2_f64 * t11365 * t1695 * t11129 + 0.10526802520742363173e2_f64 * t3401 * t4858 * t3377 + 0.51947577317044391277e2_f64 * t3401 * t15218 * t3395 + 0.30762056574649219973e4_f64 * t11310 * t51651 * t3377 + 0.17315859105681463759e2_f64 * t3401 * t4861 * t11399 + 0.91082604192152556044e5_f64 * t44223 * t1694 * t43692 * t11129 + 0.96491876992155210402e2_f64 * t15146 * t11441;
    (t51641, t51664)
}
