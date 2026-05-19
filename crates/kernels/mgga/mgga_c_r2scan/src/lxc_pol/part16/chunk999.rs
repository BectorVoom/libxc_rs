//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 999/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk999<F: Float>(t11843: F, t11845: F, t11866: F, t11876: F, t11886: F, t11502: F, t11506: F, t11554: F, t986: F, t3276: F, t3275: F, t11540: F, t3579: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t12192 = F::cast_from(0.23115257973478049502e0_f64) * t11843;
    let t12193 = F::cast_from(0.12805040077930161442e0_f64) * t11845;
    let t12230 = F::new(2.0) / F::new(3.0) * t11866;
    let t12235 = F::new(2.0) / F::new(3.0) * t11876;
    let t12238 = F::new(4.0) / F::new(3.0) * t11886;
    let t12381 = t11506 * t11502;
    let t12382 = F::new(3.0) / F::new(2.0) * t12381;
    let t12383 = t11554 * t986;
    let t12384 = t3276 * t12383;
    let t12385 = t3275 * t12384;
    let t12386 = F::new(5.0) / F::new(8.0) * t12385;
    let t12387 = t3579 * t11540;
    (t12192, t12193, t12230, t12235, t12238, t12382, t12383, t12384, t12386, t12387)
}
