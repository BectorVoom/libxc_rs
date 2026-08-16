//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1129/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1129<F: Float>(t39642: F, t39721: F, t39723: F, t39816: F, t39846: F, t39882: F, t39906: F, t39977: F, t40070: F, t40109: F, t40137: F, t40220: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t41480 = F::cast_from(0.11708928647259339622e0_f64) * t39642;
    let t41518 = F::cast_from(0.57829097596741960691e-3_f64) * t39721;
    let t41519 = F::cast_from(0.16262400898971305031e-3_f64) * t39723;
    let t41570 = F::cast_from(0.11902492299418487743e0_f64) * t39816;
    let t41582 = F::cast_from(0.84755945902752848174e0_f64) * t39846;
    let t41600 = F::cast_from(0.45022119329691164871e0_f64) * t39882;
    let t41609 = F::cast_from(0.13506635798907349462e1_f64) * t39906;
    let t41641 = F::cast_from(0.42683466926433871473e0_f64) * t39977;
    let t41680 = F::cast_from(0.11902492299418487743e0_f64) * t40070;
    let t41699 = F::cast_from(0.84755945902752848174e0_f64) * t40109;
    let t41711 = F::cast_from(0.84755945902752848174e0_f64) * t40137;
    let t41750 = F::cast_from(0.45022119329691164871e0_f64) * t40220;
    (t41480, t41518, t41519, t41570, t41582, t41600, t41609, t41641, t41680, t41699, t41711, t41750)
}
