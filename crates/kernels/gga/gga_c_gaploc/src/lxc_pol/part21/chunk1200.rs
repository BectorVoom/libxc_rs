//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1200/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1200<F: Float>(t11977: F, t11983: F, t12014: F, t1340: F, t1344: F, t2268: F, t30009: F, t30014: F, t30049: F, t31737: F, t31755: F, t31758: F, t31760: F, t31766: F, t31772: F, t31777: F, t3808: F, t38362: F, t6313: F) -> (F,) {
    let t38368 = -t30009 - t30014 - t31737 - 0.63233348079280332442e-2 * t3808 * t12014 - 0.19918504644973304719e0 * t2268 * t11977 * t1344 + 0.34146007962811379518e0 * t2268 * t38362 * t1340 + 0.15176003539027279786e0 * t6313 * t11983 - t30049 + t31755 - t31758 - t31760 + t31766 - t31772 + t31777;
    (t38368,)
}
