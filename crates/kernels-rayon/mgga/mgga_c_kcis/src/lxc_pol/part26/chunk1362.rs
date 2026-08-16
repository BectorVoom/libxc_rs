//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1362/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1362(t102115: f64, t102118: f64, t102121: f64, t102124: f64, t102127: f64, t102129: f64, t102155: f64, t103114: f64, t103119: f64, t12231: f64, t1928: f64, t27369: f64, t28353: f64, t98308: f64, t98344: f64, t990: f64) -> f64 {
    let t103366 = 0.99024918276041666664e-4_f64 * t12231 * t1928 * t990 * t28353 - 0.2653111111111111111e-1_f64 * t102115 + 0.88437037037037037033e-2_f64 * t102118 - 0.88437037037037037034e-2_f64 * t102121 + 0.17687407407407407407e-1_f64 * t102124 - 0.33163888888888888888e-2_f64 * t102127 + 0.10297067901234567901e-3_f64 * t98308 + 0.1621345679012345679e-1_f64 * t102129 - 0.44218518518518518516e-2_f64 * t98344 + 0.61836467013888888889e-4_f64 * t27369 * t103114 - 0.12367293402777777778e-3_f64 * t27369 * t103119 - 0.44218518518518518516e-2_f64 * t102155;
    t103366
}
