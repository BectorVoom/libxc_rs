//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 673/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk673<F: Float>(t1238: F, t1761: F, t4945: F, t498: F, t5055: F, t6151: F, t6153: F, t6239: F, t6244: F, t6268: F, t1763: F, t1256: F, t193: F, t336: F, t3640: F, t5985: F, t5987: F, t5991: F, t6023: F, t6026: F, t6092: F, t6094: F, t6096: F, t6100: F, t6104: F, t6108: F) -> (F, F, F) {
    let t6270 = F::cast_from(2.0_f64) * t1238 * t6244 - t1238 * t6268 - F::cast_from(2.0_f64) * t1761 * t4945 - F::cast_from(2.0_f64) * t1761 * t5055 + t498 * t6151 + F::cast_from(2.0_f64) * t498 * t6153 + t498 * t6239;
    let t6274 = t1763 * t1763;
    let t6278 = t1256 * t193 * t336 * t6270 - t193 * t336 * t3640 * t6274 - t5985 + t5987 - t5991 + t6023 + t6026 + t6092 + t6094 - t6096 + t6100 - t6104 - t6108;
    (t6270, t6274, t6278)
}
