//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 379/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk379<F: Float>(t1716: F, t738: F, t271: F, t341: F, t667: F, t656: F, t1097: F, t19: F, t252: F, t1: F, t664: F, t1112: F, t1114: F, t1116: F) -> (F, F, F, F, F, F, F) {
    let t1717 = t738 * t1716;
    let t1735 = t341 * t271;
    let t1741 = t667 * t667;
    let t1742 = t656 * t1741;
    let t1747 = t1097 * t252 * t19;
    let t1751 = t341 * t664 * t1;
    let t1759 = -F::new(0.99474444444444444447e-4) * t1112 + F::new(0.19894888888888888889e-3) * t1114 + F::new(0.52442777777777777777e-2) * t1116;
    (t1717, t1735, t1741, t1742, t1747, t1751, t1759)
}
