//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1248/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1248<F: Float>(t7908: F, t94227: F, t94353: F, t98146: F, t98344: F, t98347: F, t98350: F, t98353: F, t98357: F, t98361: F, t98365: F, t98370: F, t98373: F) -> F {
    let t98375 = -F::cast_from(0.44218518518518518517e-2_f64) * t98344 + F::cast_from(0.66327777777777777776e-2_f64) * t98347 - F::cast_from(0.33163888888888888888e-2_f64) * t98350 + F::cast_from(0.66327777777777777776e-2_f64) * t98353 + F::cast_from(0.15445601851851851852e-3_f64) * t94353 + F::cast_from(0.55273148148148148146e-2_f64) * t98357 - F::cast_from(0.12367293402777777778e-3_f64) * t94227 * t98361 - F::cast_from(0.61836467013888888888e-4_f64) * t98365 + F::cast_from(0.13901041666666666667e-2_f64) * t7908 * t98146 - F::cast_from(0.33163888888888888888e-2_f64) * t98370 - F::cast_from(0.11054629629629629629e-2_f64) * t98373;
    t98375
}
