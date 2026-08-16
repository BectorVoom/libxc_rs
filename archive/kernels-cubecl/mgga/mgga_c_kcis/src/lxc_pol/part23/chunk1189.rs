//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1189/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1189<F: Float>(t3245: F, t7935: F, t18210: F, t2237: F, t27395: F, t27402: F, t16937: F, t27358: F, t27369: F, t10470: F, t2244: F, t27339: F, t94469: F) -> (F, F, F, F, F, F, F, F) {
    let t94539 = t3245 * t7935;
    let t94546 = t2237 * t18210 * t27395;
    let t94554 = t2237 * t18210 * t27402;
    let t94585 = t16937 * t27358;
    let t94586 = t27369 * t94585;
    let t94588 = t10470 * t2244;
    let t94589 = F::cast_from(0.73697530864197530862e-3_f64) * t94588;
    let t94592 = t27339 * t94469;
    (t94539, t94546, t94554, t94585, t94586, t94588, t94589, t94592)
}
