//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1301/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1301(t10756: f64, t10806: f64, t10813: f64, t10814: f64, t10828: f64, t10829: f64, t2856: f64, t2889: f64, t2905: f64, t2930: f64, t2932: f64, t311: f64, t41733: f64, t41827: f64, t41987: f64, t42123: f64, t42128: f64, t42145: f64, t42148: f64, t42149: f64, t42154: f64, t42172: f64, t42187: f64, t42203: f64, t42218: f64, t42226: f64, t42228: f64, t42233: f64, t42235: f64, t42238: f64, t42241: f64, t42253: f64, t42266: f64, t924: f64, t932: f64, t951: f64) -> f64 {
    let t42270 = 0.1929837539843104208e3_f64 * t42123 * t2889 + 4.0_f64 * t2856 * t10806 - 0.4155806185363551302e3_f64 * t42128 * t10829 + 0.6233709278045326953e3_f64 * t10756 * t41827 * t2932 - 0.14035736694323150897e2_f64 * t10828 * t41827 * t951 - 0.35089341735807877242e1_f64 * t2905 * t41733 * t951 + 0.51947577317044391277e2_f64 * t2930 * t41733 * t2932 + t42145 - t42148 + 0.82761620670837440481e4_f64 * t42149 * t10814 - 0.24828486201251232145e5_f64 * t42154 * t41987 * t10813 + 1.0_f64 * t924 * (t42172 + t42187 + t42203 + t42218) * t932 + 0.19964560303604640732e6_f64 * t42226 * t41987 * t42228 + t42233 - t42235 + t42238 + t42241 - 0.310907e-1_f64 * (t42253 + t42266) * t311;
    t42270
}
