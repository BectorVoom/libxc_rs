//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1264/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1264<F: Float>(t40005: F, t4819: F, t16858: F, t3657: F, t56676: F, t56681: F, t56686: F, t56693: F, t56939: F, t56941: F, t56945: F, t56948: F, t56950: F) -> (F, F, F) {
    let t56952 = F::new(0.96490945932906628932e2) * t40005 * t4819;
    let t56954 = F::new(4.0) * t3657 * t16858;
    let t56955 = -t56676 + t56681 + t56686 - t56693 + t56939 + t56941 - t56945 - t56948 + t56950 + t56952 + t56954;
    (t56952, t56954, t56955)
}
