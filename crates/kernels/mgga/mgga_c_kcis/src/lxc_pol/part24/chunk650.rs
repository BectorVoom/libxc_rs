//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 650/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk650<F: Float>(t1291: F, t2205: F, t7746: F, t7750: F, t7752: F, t7756: F, t7758: F, t7760: F, t7762: F, t7764: F) -> (F, F) {
    let t7812 = t2205 * t1291;
    let t7823 = 0.9375e-1 * t7746 - 0.9375e-1 * t7750 - 0.25e0 * t7752 + 0.625e-1 * t7756 - 0.20234375e-1 * t7758 + 0.20234375e-1 * t7760 + 0.10791666666666666667e0 * t7762 - 0.26979166666666666667e-1 * t7764;
    (t7812, t7823)
}
