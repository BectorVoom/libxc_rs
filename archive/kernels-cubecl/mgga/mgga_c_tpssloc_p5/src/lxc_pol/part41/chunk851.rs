//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 851/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk851<F: Float>(t1241: F, t6267: F, t1238: F, t1761: F, t4945: F, t498: F, t5055: F, t6151: F, t6153: F, t6239: F, t6244: F, t1763: F) -> (F, F, F) {
    let t6268 = t1241 * t6267;
    let t6270 = F::cast_from(2.0_f64) * t1238 * t6244 - t1238 * t6268 - F::cast_from(2.0_f64) * t1761 * t4945 - F::cast_from(2.0_f64) * t1761 * t5055 + t498 * t6151 + F::cast_from(2.0_f64) * t498 * t6153 + t498 * t6239;
    let t6274 = t1763 * t1763;
    (t6268, t6270, t6274)
}
