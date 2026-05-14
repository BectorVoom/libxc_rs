//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 609/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk609<F: Float>(t6683: F, t6685: F, t6687: F, t6691: F, t6694: F, t6698: F, t6702: F, t6706: F, t6710: F, t6712: F, t6714: F, t6718: F, t6721: F, t6725: F, t6729: F, t6733: F) -> (F,) {
    let t6879 = 0.9375e-1 * t6683 - 0.1875e0 * t6685 + 0.125e0 * t6687 + 0.1875e0 * t6691 - 0.125e0 * t6694 - 0.9375e-1 * t6698 - 0.20833333333333333333e-1 * t6702 + 0.625e-1 * t6706 - 0.101171875e-1 * t6710 + 0.20234375e-1 * t6712 - 0.26979166666666666666e-1 * t6714 - 0.20234375e-1 * t6718 + 0.26979166666666666666e-1 * t6721 + 0.101171875e-1 * t6725 - 0.44965277777777777777e-2 * t6729 - 0.13489583333333333333e-1 * t6733;
    (t6879,)
}
