//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 342/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk342<F: Float>(t174: F, t1139: F, t1204: F, t1278: F, t1282: F, t1291: F, t187: F, t437: F, t833: F, t447: F, t637: F, t237: F, t318: F, t451: F, zeta_threshold: F) -> (F, F, F, F) {
    let t175 = t174 <= zeta_threshold;
    let t1295 = t1139 - t1204 + t187 * (t1278 * t437 - t1282 * t1291 - t1139 + t1204);
    let t1299 = piecewise3::<f64>(t175, F::new(0.0), t833);
    let t1300 = t447 * t1299;
    let t1301 = t1300 * t637;
    let t1305 = t237 * t318 * t451;
    (t1295, t1300, t1301, t1305)
}
