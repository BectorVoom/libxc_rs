//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1350/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1350<F: Float>(t15471: F, t26955: F, t26963: F, t27014: F, t28102: F, t28211: F, t5329: F, t7788: F, t7794: F, t8087: F, t8095: F, t92604: F, t92657: F, t92948: F, t93028: F, t95828: F, t96899: F, t96902: F, t96904: F, t96910: F, t96917: F) -> F {
    let t96920 = F::new(0.69505208333333333334e-3) * t27014 * t28211 + F::new(0.34752604166666666667e-3) * t7788 * t5329 * t7794 * t15471 + F::new(0.45346742476851851853e-3) * t92948 * t8087 - F::new(0.82448622685185185185e-4) * t96899 - F::new(0.7722800925925925926e-4) * t96902 + t96904 - F::new(0.38691203703703703704e-2) * t95828 - F::new(0.18534722222222222222e-2) * t92604 * t8095 + F::new(0.2782641015625e-3) * t26955 * t96910 + F::new(0.185671721767578125e-4) * t92657 * t96910 + F::new(0.30918233506944444444e-4) * t93028 * t28102 + F::new(0.23168402777777777778e-3) * t96917 * t26963;
    t96920
}
