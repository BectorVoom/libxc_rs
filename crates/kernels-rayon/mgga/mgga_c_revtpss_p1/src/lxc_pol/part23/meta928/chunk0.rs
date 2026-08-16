//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3021/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3021(t24042: f64, t359: f64, t1024: f64, t1082: f64, t11788: f64, t12127: f64, t16506: f64, t16523: f64, t19483: f64, t19556: f64, t19566: f64, t19572: f64, t19580: f64, t20119: f64, t24075: f64, t24084: f64, t3204: f64, t4757: f64, t4954: f64, t4996: f64, t5009: f64, t55991: f64, t67599: f64, t79084: f64, t79175: f64, t80028: f64, t999: f64) -> f64 {
    let t80396 = t359 * t24042;
    let t80425 = 0.19756347548806534796e1_f64 * t19566 * t5009 - 0.65854491829355115987e0_f64 * t1024 * t80396 * t999 + 0.19756347548806534796e1_f64 * t12127 * t67599 * t79175 + 0.39512695097613069592e1_f64 * t3204 * t19556 * t4757 + 0.39512695097613069591e1_f64 * t11788 * t24075 - 0.65854491829355115987e0_f64 * t1024 * t1082 * t80028 + 0.19756347548806534796e1_f64 * t4954 * t20119 + 0.19756347548806534796e1_f64 * t55991 * t19580 - 0.19756347548806534796e1_f64 * t16523 * t24084 - 0.19756347548806534796e1_f64 * t16506 * t24084 + 0.39512695097613069591e1_f64 * t3204 * t1082 * t79084 - 0.19756347548806534796e1_f64 * t4996 * t19572 * t19483;
    t80425
}
