//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1029/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1029<F: Float>(t31142: F, t8810: F, t7440: F, t8803: F, t31693: F, t31700: F, t31708: F, t1181: F, t4623: F, t604: F, t7426: F, t30090: F, t8897: F) -> (F, F, F, F, F, F, F) {
    let t36041 = t31142 * t8810;
    let t36065 = t7440 * t8803;
    let t36070 = F::cast_from(0.14291339372689912324e-2_f64) * t31693;
    let t36072 = F::cast_from(0.28582678745379824648e-3_f64) * t31700;
    let t36075 = F::cast_from(0.57165357490759649296e-3_f64) * t31708;
    let t36081 = t7426 * t1181 * t604 * t4623;
    let t36083 = t30090 * t8897;
    (t36041, t36065, t36070, t36072, t36075, t36081, t36083)
}
