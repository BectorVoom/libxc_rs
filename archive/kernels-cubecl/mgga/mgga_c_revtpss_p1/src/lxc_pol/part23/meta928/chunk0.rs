//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3021/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3021<F: Float>(t24042: F, t359: F, t1024: F, t1082: F, t11788: F, t12127: F, t16506: F, t16523: F, t19483: F, t19556: F, t19566: F, t19572: F, t19580: F, t20119: F, t24075: F, t24084: F, t3204: F, t4757: F, t4954: F, t4996: F, t5009: F, t55991: F, t67599: F, t79084: F, t79175: F, t80028: F, t999: F) -> F {
    let t80396 = t359 * t24042;
    let t80425 = F::cast_from(0.19756347548806534796e1_f64) * t19566 * t5009 - F::cast_from(0.65854491829355115987e0_f64) * t1024 * t80396 * t999 + F::cast_from(0.19756347548806534796e1_f64) * t12127 * t67599 * t79175 + F::cast_from(0.39512695097613069592e1_f64) * t3204 * t19556 * t4757 + F::cast_from(0.39512695097613069591e1_f64) * t11788 * t24075 - F::cast_from(0.65854491829355115987e0_f64) * t1024 * t1082 * t80028 + F::cast_from(0.19756347548806534796e1_f64) * t4954 * t20119 + F::cast_from(0.19756347548806534796e1_f64) * t55991 * t19580 - F::cast_from(0.19756347548806534796e1_f64) * t16523 * t24084 - F::cast_from(0.19756347548806534796e1_f64) * t16506 * t24084 + F::cast_from(0.39512695097613069591e1_f64) * t3204 * t1082 * t79084 - F::cast_from(0.19756347548806534796e1_f64) * t4996 * t19572 * t19483;
    t80425
}
