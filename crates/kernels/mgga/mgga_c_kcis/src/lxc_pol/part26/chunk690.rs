//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 690/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk690<F: Float>(t1629: F, t1636: F, t187: F, t2268: F, t4475: F, t4480: F, t633: F, t7939: F, t7941: F, t7942: F, t7945: F, t7963: F, t7996: F, t7998: F, t8001: F, t8010: F) -> (F,) {
    let t8014 = t7939 - t7941 - t7942 + t7945 - t7963 + t187 * (-t1629 * t8010 - t1636 * t7998 - t2268 * t4475 + 2.0 * t4480 * t8001 + t633 * t7996 - t7939 + t7941 + t7942 - t7945 + t7963);
    (t8014,)
}
