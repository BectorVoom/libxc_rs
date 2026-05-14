//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1390/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1390<F: Float>(t18427: F, t18430: F, t18843: F, t22230: F, t22233: F, t22236: F, t27262: F, t27289: F, t27295: F, t352: F, t2196: F, t3734: F, t2199: F, t18851: F, t2257: F, t22575: F, t2296: F, t2298: F, t2318: F, t2320: F, t27453: F, t27521: F, t27523: F, t27525: F, t27527: F, t27530: F, t27830: F, t27850: F, t27905: F, t27908: F, t27911: F, t27912: F, t27916: F, t3083: F, t3780: F, t8068: F, t8129: F, t871: F, t890: F) -> (F, F, F) {
    let t27930 = 0.621814e-1 * (t18843 - 0.11080740740740740741e0 * t18427 + 0.23744444444444444444e-1 * t18430 - 0.11080740740740740741e0 * t22230 + 0.94977777777777777776e-1 * t22233 - 0.35616666666666666666e-1 * t22236 + 0.23744444444444444444e-1 * t27295 - 0.35616666666666666666e-1 * t27262 + 0.53425e-1 * t27289) * t352;
    let t27937 = t3734 * t2196;
    let t27939 = 2.0 * t27937 * t2199;
    let t27944 = -t27521 - t27523 + t27525 - t27527 - t27530 + 24.0 * t22575 * t8129 - t27850 - t27905 - t27908 - t27911 - 0.11696447245269292414e1 * t27912 * t2298 - t27916 - 4.0 * t2257 * t27830 * t871 + t27930 - 0.23392894490538584828e1 * t2296 * t27453 * t890 + 0.34631718211362927518e2 * t2318 * t27453 * t2320 + t27939 + 2.0 * t3083 * t8068 - 2.0 * t18851 * t3780;
    (t27930, t27939, t27944)
}
