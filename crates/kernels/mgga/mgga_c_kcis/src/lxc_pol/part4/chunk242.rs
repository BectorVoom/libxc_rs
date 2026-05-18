//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 242/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk242<F: Float>(t227: F, t224: F, t806: F, t810: F, t813: F, t816: F, t819: F, t824: F) -> (F, F, F, F) {
    let t897 = t227 * t227;
    let t898 = F::new(1.0) / t897;
    let t899 = t224 * t898;
    let t906 = F::new(0.1875e0) * t806 - F::new(0.1875e0) * t810 - F::new(0.375e0) * t813 - F::new(0.4046875e-1) * t816 + F::new(0.4046875e-1) * t819 + F::new(0.161875e0) * t824;
    (t897, t898, t899, t906)
}
