//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 755/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk755<F: Float>(t24682: F, t27628: F, t1409: F, t2132: F, t2136: F, t210: F, t7998: F, t1193: F, t8020: F, t52: F, t8027: F, t461: F, t7573: F) -> (F, F, F, F, F, F) {
    let t27629 = t24682 * t27628;
    let t27650 = t2132 * t1409;
    let t27651 = t27650 * t2136;
    let t27674 = t7998 * t210;
    let t27677 = t8020 * t1193;
    let t27680 = t8027 * t52;
    let t27681 = t27680 * t2136;
    let t27683 = t7573 * t461;
    (t27629, t27651, t27674, t27677, t27681, t27683)
}
