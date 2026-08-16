//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1128/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1128<F: Float>(t6366: F, t949: F, t2986: F, t4740: F, t5250: F, t1226: F, t6428: F, t6406: F, t9825: F, t4764: F, t45: F, t6400: F) -> (F, F, F, F, F) {
    let t18997 = t6366 * t949;
    let t18999 = F::cast_from(6.0_f64) * t2986 * t18997;
    let t19006 = t4740 * t5250;
    let t19011 = t6428 * t1226;
    let t19018 = t9825 * t6406;
    let t19019 = t19018 * t4764;
    let t19022 = t45 * t6400;
    (t18999, t19006, t19011, t19019, t19022)
}
