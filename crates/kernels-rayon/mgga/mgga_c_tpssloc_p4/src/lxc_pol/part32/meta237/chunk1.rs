//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1072/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1072(t1238: f64, t1761: f64, t4945: f64, t498: f64, t5055: f64, t6151: f64, t6153: f64, t6239: f64, t6244: f64, t6268: f64, t1763: f64, t1256: f64, t193: f64, t336: f64, t3640: f64, t5985: f64, t5987: f64, t5991: f64, t6023: f64, t6026: f64, t6092: f64, t6094: f64, t6096: f64, t6100: f64, t6104: f64, t6108: f64) -> (f64, f64, f64) {
    let t6270 = 2.0_f64 * t1238 * t6244 - t1238 * t6268 - 2.0_f64 * t1761 * t4945 - 2.0_f64 * t1761 * t5055 + t498 * t6151 + 2.0_f64 * t498 * t6153 + t498 * t6239;
    let t6274 = t1763 * t1763;
    let t6278 = t1256 * t193 * t336 * t6270 - t193 * t336 * t3640 * t6274 - t5985 + t5987 - t5991 + t6023 + t6026 + t6092 + t6094 - t6096 + t6100 - t6104 - t6108;
    (t6270, t6274, t6278)
}
