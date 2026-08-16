//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 724/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk724<F: Float>(t1457: F, t6321: F, t4752: F, t494: F, t1344: F, t1645: F, t188: F, t6316: F, t1340: F, t2345: F, t4673: F, t493: F, t6519: F) -> (F, F, F, F, F, F, F) {
    let t6734 = t1457 * t6321;
    let t6737 = t4752 * t494;
    let t6740 = t1645 * t1344;
    let t6743 = t188 * t6316;
    let t6744 = t1645 * t1340;
    let t6747 = t4673 * t2345;
    let t6750 = t493 * t6519;
    (t6734, t6737, t6740, t6743, t6744, t6747, t6750)
}
