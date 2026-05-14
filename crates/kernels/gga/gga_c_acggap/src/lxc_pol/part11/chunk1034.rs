//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1034/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1034<F: Float>(t142: F, t4586: F, t7436: F, t7440: F, t8803: F, t2030: F, t5183: F, t7815: F, t31693: F, t31700: F, t31708: F, t4680: F, t7564: F, t8449: F, t1181: F, t4623: F, t604: F, t7426: F) -> (F, F, F, F, F, F, F, F) {
    let t36063 = t7436 * t142 * t4586;
    let t36065 = t7440 * t8803;
    let t36066 = 11.0 / 288.0 * t36065;
    let t36068 = t2030 * t7815 * t5183;
    let t36070 = 0.14291339372689912324e-2 * t31693;
    let t36072 = 0.28582678745379824648e-3 * t31700;
    let t36075 = 0.57165357490759649296e-3 * t31708;
    let t36077 = t7564 * t4680 * t8449;
    let t36081 = t7426 * t1181 * t604 * t4623;
    (t36063, t36066, t36068, t36070, t36072, t36075, t36077, t36081)
}
