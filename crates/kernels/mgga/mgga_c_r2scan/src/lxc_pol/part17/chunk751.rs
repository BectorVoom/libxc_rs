//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 751/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk751<F: Float>(t1607: F, t5100: F, t512: F, t6101: F, t507: F, t1591: F, t2168: F, t1541: F, t545: F, t548: F, t110: F, t6189: F) -> (F, F, F, F, F, F) {
    let t6420 = t5100 * t1607;
    let t6422 = t512 * t6101;
    let t6424 = F::new(0.174549769648958674e0) * t6422 * t507;
    let t6425 = t1591 * t2168;
    let t6448 = t545 * t1541;
    let t6449 = t6448 * t548;
    let t6461 = t6189 * t110;
    (t6420, t6424, t6425, t6448, t6449, t6461)
}
