//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1673/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1673<F: Float>(t225: F, t5213: F, t1807: F, t3879: F, t5211: F, t1332: F, t5343: F, t1372: F, t1824: F, t5250: F, t5286: F, t562: F) -> (F, F, F, F, F, F, F) {
    let t16022 = t5213 * t225;
    let t16028 = t1807 * t3879;
    let t16030 = t5211 * t225;
    let t16033 = t1332 * t5343;
    let t16036 = t1372 * t1824;
    let t16037 = t16036 * t5250;
    let t16040 = t562 * t5286;
    (t16022, t16028, t16030, t16033, t16036, t16037, t16040)
}
