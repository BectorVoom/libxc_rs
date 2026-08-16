//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1309/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1309(t1203: f64, t1214: f64, t1248: f64, t12651: f64, t1269: f64, t12696: f64, t1287: f64, t1294: f64, t2151: f64, t2152: f64, t26889: f64, t26895: f64, t26940: f64, t26944: f64, t26949: f64, t26950: f64, t26951: f64, t26958: f64, t26962: f64, t26983: f64, t26987: f64, t26994: f64, t3569: f64, t5458: f64, t7602: f64, t7632: f64, t7636: f64, t7637: f64, t7643: f64, t7645: f64, t7652: f64, t97011: f64, t97019: f64, t97034: f64, t97041: f64, t97050: f64, t97066: f64, t97067: f64) -> f64 {
    let t97072 = 0.26020884564615598386e1_f64 * t26895 * t97011 * t5458 + 0.39512695097613069591e1_f64 * t97019 * t3569 + 0.52041769129231196772e1_f64 * t7636 * t7652 * t26962 * t1294 + 0.52041769129231196772e1_f64 * t26994 * t7637 * t26958 * t1203 + 0.19756347548806534796e1_f64 * t7602 * t12651 + 0.39512695097613069591e1_f64 * t7632 * t12696 + 0.26020884564615598386e1_f64 * t97034 * t7645 - 0.26020884564615598386e1_f64 * t26983 * t1269 * t2152 - 0.78062653693846795158e1_f64 * t97041 * t26950 * t1248 * t1287 - 0.52041769129231196772e1_f64 * t26889 * t26940 * t1248 * t1287 - 0.78062653693846795158e1_f64 * t97050 * t26951 + 0.15612530738769359031e2_f64 * t26949 * t7652 * t26950 * t1294 - 0.10408353825846239354e2_f64 * t7643 * t7652 * t26944 * t1214 + 0.52041769129231196772e1_f64 * t7636 * t7652 * t26987 * t1203 - 0.20816707651692478709e2_f64 * t97066 * t2151 * t97067 * t1214;
    t97072
}
