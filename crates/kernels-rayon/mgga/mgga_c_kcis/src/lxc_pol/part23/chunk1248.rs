//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1248/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1248(t7908: f64, t94227: f64, t94353: f64, t98146: f64, t98344: f64, t98347: f64, t98350: f64, t98353: f64, t98357: f64, t98361: f64, t98365: f64, t98370: f64, t98373: f64) -> f64 {
    let t98375 = -0.44218518518518518517e-2_f64 * t98344 + 0.66327777777777777776e-2_f64 * t98347 - 0.33163888888888888888e-2_f64 * t98350 + 0.66327777777777777776e-2_f64 * t98353 + 0.15445601851851851852e-3_f64 * t94353 + 0.55273148148148148146e-2_f64 * t98357 - 0.12367293402777777778e-3_f64 * t94227 * t98361 - 0.61836467013888888888e-4_f64 * t98365 + 0.13901041666666666667e-2_f64 * t7908 * t98146 - 0.33163888888888888888e-2_f64 * t98370 - 0.11054629629629629629e-2_f64 * t98373;
    t98375
}
