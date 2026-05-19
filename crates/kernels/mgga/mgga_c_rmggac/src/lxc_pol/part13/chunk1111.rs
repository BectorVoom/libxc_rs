//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1111/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1111<F: Float>(t2227: F, t570: F, t4895: F, t699: F, t118: F, t305: F, t321: F, t35980: F, t35989: F, t41393: F, t41395: F, t41402: F, t41405: F, t41409: F, t41412: F, t41436: F, t43685: F, t5148: F) -> (F, F, F) {
    let t44157 = t2227 * t570;
    let t44162 = t699 * t4895;
    let t44168 = F::cast_from(0.8980681276397856423e-1_f64) * t41393 + F::cast_from(0.71845450211182851384e0_f64) * t41395 + F::cast_from(0.2727466165424534173e0_f64) * t41402 + F::cast_from(0.32729593985094410076e0_f64) * t41405 + F::cast_from(0.81823984962736025192e-1_f64) * t41409 - F::cast_from(0.16364796992547205038e0_f64) * t41412 - F::cast_from(0.95793933614910468512e0_f64) * t35980 - F::cast_from(0.23948483403727617128e0_f64) * t5148 * t44157 * t321 - F::cast_from(0.15965655602485078085e0_f64) * t35989 + F::cast_from(0.59871208509319042821e-1_f64) * t305 * t44162 - F::cast_from(0.79828278012425390428e-1_f64) * t118 * t43685 + F::cast_from(0.11974241701863808564e0_f64) * t41436;
    (t44157, t44162, t44168)
}
