//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 764/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk764<F: Float>(t157: F, t609: F, t929: F, t2152: F, t2124: F, t310: F, t611: F, t848: F, t315: F, t7941: F) -> (F, F, F, F) {
    let t7953 = t609 * t929 * t157;
    let t7954 = t2152 * t7953;
    let t7957 = t310 * t2124;
    let t7962 = F::new(0.65854491829355115987e0) * t848 * t611;
    let t7963 = t315 * t7941;
    (t7954, t7957, t7962, t7963)
}
