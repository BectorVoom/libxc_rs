//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2579/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2579<F: Float>(t1703: F, t65288: F, t71543: F, t71545: F, t71547: F, t71655: F, t71657: F, t71697: F, t72061: F, t72065: F, t72067: F, t72071: F) -> (F, F) {
    let t72073 = F::cast_from(0.17544670867903938621e1_f64) * t65288 * t1703;
    let t72074 = -t72061 - t72065 + t71543 - t71545 + t71547 + t71655 + t71657 + t72067 - t72071 - t72073 - t71697;
    (t72073, t72074)
}
