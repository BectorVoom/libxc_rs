//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 692/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk692<F: Float>(t1142: F, t8081: F, t2192: F, t5345: F, t1856: F, t7773: F, t5329: F) -> (F, F, F, F) {
    let t8082 = t1142 * t8081;
    let t8083 = t5345 * t2192;
    let t8086 = t7773 * t1856;
    let t8087 = t5329 * t8086;
    (t8082, t8083, t8086, t8087)
}
