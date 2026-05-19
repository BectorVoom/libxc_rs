//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1080/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1080<F: Float>(t36063: F, t36065: F, t36072: F, t36074: F, t36078: F, t36088: F, t36090: F, t36092: F, t41210: F, t41213: F, t41216: F, t41219: F, t41222: F, t41225: F, t41228: F, t41230: F) -> F {
    let t43550 = -F::cast_from(0.45158592333657918154e-2_f64) * t36063 - F::cast_from(0.12122071846331262991e0_f64) * t36065 - F::cast_from(0.2419210303588817044e-2_f64) * t36072 - F::cast_from(0.36366215538993788972e-1_f64) * t36074 + F::cast_from(0.24244143692662525982e-1_f64) * t36078 + F::cast_from(0.35481751119302649978e-2_f64) * t36088 - F::cast_from(0.41395376305853091641e-2_f64) * t36090 - F::cast_from(0.4838420607177634088e-3_f64) * t36092 + F::cast_from(0.2727466165424534173e-1_f64) * t41210 + F::cast_from(0.13637330827122670865e-1_f64) * t41213 - F::cast_from(0.5454932330849068346e-1_f64) * t41216 - F::cast_from(0.2727466165424534173e-1_f64) * t41219 - F::cast_from(0.5454932330849068346e-1_f64) * t41222 - F::cast_from(0.2727466165424534173e-1_f64) * t41225 + F::cast_from(0.90915538847484472432e-1_f64) * t41228 + F::cast_from(0.3540307761349488357e-2_f64) * t41230;
    t43550
}
