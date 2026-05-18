//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1204/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1204<F: Float>(t13866: F, t13869: F, t13871: F, t13874: F, t13876: F, t13878: F, t13956: F, t14035: F, t14038: F, t14042: F, t14044: F, t14046: F, t14049: F) -> F {
    let t15464 = t13866 - t13869 - t13871 - t13874 - t13876 - t13878 - t13956 - t14035 - t14038 - t14042 + t14044 - t14046 + t14049;
    t15464
}
