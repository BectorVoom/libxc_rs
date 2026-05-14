//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1240/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1240<F: Float>(t28877: F, t28880: F, t28883: F, t28885: F, t28887: F, t28890: F, t28892: F, t28894: F, t28896: F, t28899: F, t28901: F, t28904: F, t24556: F, t24559: F, t24562: F, t24658: F, t24661: F, t24664: F, t24667: F, t24670: F, t24673: F, t28907: F, t28917: F, t28919: F) -> (F, F) {
    let t29010 = 0.1151859375e0 * t28877 - 0.76790625e-1 * t28880 - 0.3560484375e1 * t28883 + 0.142419375e1 * t28885 - 0.1898925e1 * t28887 - 0.1898925e1 * t28890 - 0.9494625e0 * t28892 - 0.76790625e-1 * t28894 + 0.3071625e0 * t28896 + 0.3071625e0 * t28899 + 0.15358125e0 * t28901 - 0.3071625e0 * t28904;
    let t29023 = 0.5696775e1 * t28907 + 0.3071625e0 * t28917 + 0.1898925e1 * t28919 - 0.1860237037037037037e1 * t24556 + 0.15944888888888888889e1 * t24559 - 0.59793333333333333334e0 * t24562 + 0.10954222222222222222e1 * t24658 + 0.10954222222222222222e1 * t24661 - 0.14605629629629629629e1 * t24664 - 0.32862666666666666666e0 * t24667 - 0.65725333333333333332e0 * t24670 - 0.32862666666666666666e0 * t24673;
    (t29010, t29023)
}
