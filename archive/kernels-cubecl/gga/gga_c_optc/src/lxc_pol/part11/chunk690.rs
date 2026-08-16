//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 690/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk690<F: Float>(t601: F, t6825: F, t518: F, t622: F, t84: F, t596: F, t120: F, t2086: F, t105: F, t2156: F, t635: F, t127: F, t2024: F) -> (F, F, F, F, F, F, F) {
    let t6827 = F::cast_from(0.35089340384731224426e1_f64) * t601 * t6825;
    let t6838 = t518 * t622 * t84;
    let t6840 = F::cast_from(0.56969282336565386482e-3_f64) * t596 * t6838;
    let t6855 = t120 * t2086;
    let t6875 = t105 * t2156;
    let t6876 = t6875 * t635;
    let t6879 = t2024 * t127;
    (t6827, t6838, t6840, t6855, t6875, t6876, t6879)
}
