//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 208/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk208<F: Float>(t224: F, t898: F, t806: F, t810: F, t813: F, t816: F, t819: F, t824: F) -> (F, F) {
    let t899 = t224 * t898;
    let t906 = 0.1875e0 * t806 - 0.1875e0 * t810 - 0.375e0 * t813 - 0.4046875e-1 * t816 + 0.4046875e-1 * t819 + 0.161875e0 * t824;
    (t899, t906)
}
