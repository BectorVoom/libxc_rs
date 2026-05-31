//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1093/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1093<F: Float>(t19623: F, t4865: F, t7046: F, t4868: F, t1545: F, t2605: F, t1548: F, t16502: F, t16508: F, t2609: F, t5089: F, t135: F, t568: F) -> (F, F, F, F, F, F, F, F, F) {
    let t19624 = F::cast_from(0.32530743900905219526e-1_f64) * t19623;
    let t19625 = t7046 * t4865;
    let t19627 = t7046 * t4868;
    let t19687 = t1545 * t2605;
    let t19688 = F::cast_from(36.0_f64) * t19687;
    let t19690 = F::cast_from(96.0_f64) * t1548 * t2605;
    let t19695 = F::cast_from(12.0_f64) * t16502;
    let t19697 = F::cast_from(144.0_f64) * t16508;
    let t19702 = t2609 * t5089;
    let t19704 = t135 * t568;
    (t19624, t19625, t19627, t19688, t19690, t19695, t19697, t19702, t19704)
}
