//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 761/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk761<F: Float>(t1734: F, t9066: F, t1743: F, t5218: F, t122: F, t1845: F, t2995: F, t3001: F, t3060: F, t3008: F, t102: F, t505: F) -> (F, F, F, F, F, F, F) {
    let t9067 = t1734 * t9066;
    let t9068 = t1743 * t9067;
    let t9069 = t9068 * t5218;
    let t9071 = t1845 * t122;
    let t9072 = t9071 * t2995;
    let t9073 = t9072 * t3001;
    let t9075 = t3060 * t2995;
    let t9076 = t9075 * t3008;
    let t9078 = t102 * t505;
    (t9067, t9068, t9069, t9071, t9073, t9076, t9078)
}
