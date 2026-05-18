//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 608/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk608<F: Float>(t1629: F, t1636: F, t187: F, t2128: F, t4475: F, t4480: F, t5896: F, t5898: F, t5899: F, t5902: F, t6049: F, t6220: F, t6222: F, t6225: F, t6256: F, t633: F) -> F {
    let t6260 = t5896 - t5898 - t5899 + t5902 - t6049 + t187 * (-t1629 * t6256 - t1636 * t6222 - t2128 * t4475 + F::new(2.0) * t4480 * t6225 + t6220 * t633 - t5896 + t5898 + t5899 - t5902 + t6049);
    t6260
}
