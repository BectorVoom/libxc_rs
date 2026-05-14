//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 730/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk730<F: Float>(t2026: F, t3393: F, t3751: F, t538: F, t5427: F, t1517: F, t1650: F, t4225: F, t1392: F, t5441: F, t1518: F, t167: F, t2029: F, t238: F, t86: F, t1979: F, t531: F) -> (F, F, F, F, F, F, F, F, F) {
    let t5966 = t3393 * t2026;
    let t5968 = t3751 * t538;
    let t5969 = t5968 * t5427;
    let t5973 = t1517 * t4225 * t1650;
    let t5976 = t1392 * t538;
    let t5977 = t5976 * t5441;
    let t5981 = t1517 * t1518 * t167;
    let t5985 = t86 * t238 * t2029;
    let t5987 = t1979 * t531;
    (t5966, t5968, t5969, t5973, t5976, t5977, t5981, t5985, t5987)
}
