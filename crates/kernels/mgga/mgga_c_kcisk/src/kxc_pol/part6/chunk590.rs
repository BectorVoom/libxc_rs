//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 590/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk590<F: Float>(t1248: F, t1720: F, t8518: F, t4876: F, t4888: F, t7076: F, t7122: F, t8684: F, t8687: F, t8690: F, t8702: F, t8709: F, t8715: F, t8717: F, t8721: F, t8724: F) -> (F, F) {
    let t8727 = t1248 * t1720 * t8518;
    let t8729 = -0.9494625e0 * t8702 + 0.1898925e1 * t8709 + t4876 + 0.19931111111111111111e0 * t7076 - 0.19931111111111111111e0 * t8684 + 0.59793333333333333334e0 * t8687 - 0.29896666666666666667e0 * t8690 + 0.15358125e0 * t8715 + 0.3071625e0 * t8717 + t4888 + 0.21908444444444444444e0 * t7122 - 0.5477111111111111111e-1 * t8721 + 0.32862666666666666666e0 * t8724 - 0.16431333333333333333e0 * t8727;
    (t8727, t8729)
}
