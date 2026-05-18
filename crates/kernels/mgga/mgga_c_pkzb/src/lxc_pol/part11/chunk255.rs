//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 255/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk255<F: Float>(t851: F, t852: F, t833: F, t819: F, t826: F) -> (F, F, F, F) {
    let t853 = t851 * t852;
    let t855 = F::new(1.0) * t833 * t853;
    let t856 = F::new(0.17123333333333333333e-1) * t819;
    let t858 = -t856 + F::new(0.5137e-1) * t826;
    (t853, t855, t856, t858)
}
