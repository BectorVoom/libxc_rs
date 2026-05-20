//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3035/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3035<F: Float>(t4746: F, t6343: F, t1000: F, t1079: F, t11187: F, t16284: F, t16312: F, t1651: F, t1652: F, t16597: F, t1680: F, t1696: F, t19341: F, t19342: F, t19381: F, t19400: F, t19428: F, t19856: F, t20151: F, t20172: F, t20204: F, t20211: F, t23583: F, t24177: F, t24178: F, t3264: F, t3268: F, t4752: F, t4758: F, t4773: F, t4778: F, t4941: F, t4946: F, t53167: F, t6244: F, t6245: F, t6259: F, t68072: F, t68144: F, t68188: F, t995: F, t999: F) -> F {
    let t80901 = t4746 * t6343;
    let t80918 = -F::cast_from(0.19756347548806534796e1_f64) * t16597 * t6259 - F::cast_from(0.19756347548806534796e1_f64) * t20204 * t4773 + F::cast_from(0.39512695097613069591e1_f64) * t53167 * t6245 - F::cast_from(0.39512695097613069591e1_f64) * t11187 * t23583 + F::cast_from(0.79025390195226139182e1_f64) * t16312 * t19428 * t19341 - F::cast_from(0.19756347548806534796e1_f64) * t4778 * t19381 - F::cast_from(0.65854491829355115987e0_f64) * t3264 * t24178 + F::cast_from(0.39512695097613069592e1_f64) * t4752 * t20172 + F::cast_from(0.19756347548806534796e1_f64) * t19856 * t1680 + F::cast_from(0.19756347548806534796e1_f64) * t995 * t1079 * t1651 * t20151 + F::cast_from(0.65854491829355115987e0_f64) * t995 * t1079 * t24177 * t999 + F::cast_from(0.79025390195226139182e1_f64) * t16284 * t19400 - F::cast_from(0.19756347548806534796e1_f64) * t80901 * t1000 - F::cast_from(0.19756347548806534796e1_f64) * t68072 * t1652 - F::cast_from(0.19756347548806534796e1_f64) * t68188 * t1696 - F::cast_from(0.39512695097613069591e1_f64) * t4778 * t19342 + F::cast_from(0.79025390195226139182e1_f64) * t16312 * t3268 * t6244 * t4946 + F::cast_from(0.79025390195226139182e1_f64) * t68144 * t4758 + F::cast_from(0.19756347548806534796e1_f64) * t20211 * t4941;
    t80918
}
