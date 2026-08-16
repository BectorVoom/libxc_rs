//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1263/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1263<F: Float>(t11063: F, t11064: F, t11067: F, t11100: F, t11101: F, t1134: F, t18258: F, t2118: F, t21346: F, t2964: F, t2989: F, t307: F, t30843: F, t30925: F, t30977: F, t3694: F, t7824: F, t786: F, t790: F, t799: F, t9647: F, t9712: F, t9713: F) -> F {
    let t30982 = -F::cast_from(0.19756347548806534796e1_f64) * t1134 * t9713 - F::cast_from(0.39512695097613069591e1_f64) * t786 * t11064 + F::cast_from(0.15805078039045227836e2_f64) * t307 * t18258 * t11063 * t799 - F::cast_from(0.11853808529283920877e2_f64) * t307 * t9647 * t2989 + F::cast_from(0.39512695097613069591e1_f64) * t786 * t11067 - F::cast_from(0.11853808529283920877e2_f64) * t21346 * t30843 * t799 + F::cast_from(0.39512695097613069591e1_f64) * t307 * t7824 * t3694 + F::cast_from(0.39512695097613069591e1_f64) * t307 * t2964 * t9712 - F::cast_from(0.65854491829355115987e0_f64) * t786 * t11101 + F::cast_from(0.13170898365871023197e1_f64) * t307 * t2118 * t11100 * t799 - F::cast_from(0.65854491829355115987e0_f64) * t307 * t790 * (t30925 + t30977);
    t30982
}
