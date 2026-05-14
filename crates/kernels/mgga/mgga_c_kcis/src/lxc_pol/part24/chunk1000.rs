//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1000/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1000<F: Float>(t1142: F, t29081: F, t2205: F, t6879: F, t1872: F, t8117: F, t20711: F, t29025: F, t29027: F, t29030: F, t29033: F, t29035: F, t29038: F, t29044: F, t3669: F, t5360: F) -> (F, F, F, F) {
    let t29082 = t1142 * t29081;
    let t29084 = t2205 * t6879;
    let t29087 = t8117 * t1872;
    let t29092 = -t20711 * t2205 + 2.0 * t29084 * t3669 + 4.0 * t29087 * t3669 - 2.0 * t5360 * t8117 - t29025 + t29027 + t29030 - t29033 + t29035 + t29038 - t29044;
    (t29082, t29084, t29087, t29092)
}
