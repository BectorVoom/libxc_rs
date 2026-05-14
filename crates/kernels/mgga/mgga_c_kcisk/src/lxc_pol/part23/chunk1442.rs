//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1442/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1442<F: Float>(t114822: F, t114829: F, t114831: F, t114837: F, t114838: F, t114840: F, t114841: F, t114844: F, t114851: F, t114853: F, t114862: F, t114866: F, t114962: F, t114970: F, t114972: F, t114975: F, t115985: F, t115986: F, t115991: F) -> (F,) {
    let t115997 = t114822 - t114829 + t114831 - t114837 - t114838 + t114840 - t114841 - t114844 - t114851 + t114853 - t114862 - t114866 + t114962 - t114970 + t114972 + t114975 - t115985 - t115986 - t115991;
    (t115997,)
}
