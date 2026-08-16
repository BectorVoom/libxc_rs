//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 925/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk925<F: Float>(t22642: F, t22643: F, t8458: F, t2006: F, t212: F, t6890: F, t22716: F, t8459: F, t22817: F, t794: F, t8462: F, t1336: F, t1338: F, t241: F, t835: F) -> (F, F, F, F, F) {
    let t113934 = F::cast_from(0.16449340668482264365e-1_f64) * t22642 * t22643 * t8458;
    let t113941 = F::cast_from(0.16449340668482264365e-1_f64) * t22642 * t212 * t2006 * t6890;
    let t113963 = F::cast_from(0.12793931631041761173e0_f64) * t22716 * t8459;
    let t113981 = t22817 * t794 * t8462;
    let t114011 = t1336 * t1338 * t835 * t241;
    (t113934, t113941, t113963, t113981, t114011)
}
