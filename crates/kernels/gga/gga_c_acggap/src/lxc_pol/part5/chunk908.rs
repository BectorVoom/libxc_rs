//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 908/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk908<F: Float>(t1032: F, t4625: F, t1181: F, t16507: F, t3361: F, t4267: F, t13502: F, t532: F, t1581: F, t3670: F, t1588: F, t1008: F, t4894: F, t14106: F, t537: F, t1576: F) -> (F, F, F, F, F, F, F, F) {
    let t16648 = t1032 * t4625;
    let t16663 = t3361 * t1181 * t4267 * t16507;
    let t16674 = t13502 * t532;
    let t16676 = t3670 * t1581;
    let t16678 = t3670 * t1588;
    let t16680 = t1008 * t4894;
    let t16686 = t14106 * t537;
    let t16688 = t3670 * t1576;
    (t16648, t16663, t16674, t16676, t16678, t16680, t16686, t16688)
}
