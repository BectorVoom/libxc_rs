//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 878/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk878<F: Float>(t42238: F, t42242: F, t42246: F, t42258: F, t9640: F, t9642: F, t9126: F, t9129: F, t9135: F, t9139: F, t9143: F, t9148: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t44444 = F::cast_from(0.1440846329149835838e-2_f64) * t42238;
    let t44445 = F::cast_from(0.1440846329149835838e-2_f64) * t42242;
    let t44446 = F::cast_from(0.1440846329149835838e-2_f64) * t42246;
    let t44450 = F::cast_from(0.39726959900411316772e-4_f64) * t42258;
    let t44466 = F::cast_from(0.4726e1_f64) * t9640;
    let t44467 = F::cast_from(0.39914139006212695214e-1_f64) * t9642;
    let t44468 = F::cast_from(0.5987120850931904282e-1_f64) * t9126;
    let t44470 = F::cast_from(0.11974241701863808564e0_f64) * t9129;
    let t44472 = F::cast_from(0.5454932330849068346e-1_f64) * t9135;
    let t44473 = F::cast_from(0.3405167991463827152e-4_f64) * t9139;
    let t44474 = F::cast_from(0.1702583995731913576e-4_f64) * t9143;
    let t44475 = F::cast_from(0.212822999466489197e-4_f64) * t9148;
    (t44444, t44445, t44446, t44450, t44466, t44467, t44468, t44470, t44472, t44473, t44474, t44475)
}
