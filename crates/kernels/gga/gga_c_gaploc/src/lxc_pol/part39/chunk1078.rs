//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1078/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1078<F: Float>(t447: F, t46867: F, t1063: F, t1064: F, t11981: F, t2293: F, t2268: F, t2343: F, t42652: F, t42655: F, t42659: F, t42661: F, t42664: F, t42671: F, t42674: F, t42675: F, t42678: F) -> (F, F, F) {
    let t46941 = t46867 * t447;
    let t46944 = F::new(0.28455006635676149599e-1) * t1063 * t1064 * t46941;
    let t46945 = t11981 * t2293;
    let t46947 = t2268 * t2343 * t46945;
    let t46949 = -t42652 + t42655 - t42659 - F::new(0.11856252764865062333e-2) * t42661 + F::new(0.11856252764865062333e-2) * t42664 - F::new(0.35568758294595186999e-2) * t42671 - t42674 - F::new(0.85365019907028448797e-1) * t42675 - F::new(0.85365019907028448797e-1) * t42678 + t46944 + F::new(0.56910013271352299198e-1) * t46947;
    (t46941, t46945, t46949)
}
