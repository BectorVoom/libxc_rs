//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 848/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk848<F: Float>(t1175: F, t236: F, t3352: F, t551: F, t8517: F, t1971: F, t3351: F, t3924: F, t5223: F, t623: F, t7262: F, t7265: F) -> (F, F, F) {
    let t39009 = t8517 * t3352 * t236 * t551 * t1175;
    let t39016 = t3351 * t1971 * t3924 * t5223;
    let t39020 = t623 * t7262;
    let t39021 = t39020 * t7265;
    (t39009, t39016, t39021)
}
