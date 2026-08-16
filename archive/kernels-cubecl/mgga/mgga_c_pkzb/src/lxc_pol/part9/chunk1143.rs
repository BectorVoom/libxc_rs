//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1143/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1143<F: Float>(t19803: F, t1009: F, t5137: F, t16638: F, t1634: F, t637: F, t1508: F, t7035: F, t496: F, t6825: F, t2562: F, t500: F) -> (F, F, F, F, F, F, F) {
    let t19804 = F::cast_from(240.0_f64) * t19803;
    let t19805 = t5137 * t1009;
    let t19806 = F::cast_from(120.0_f64) * t19805;
    let t19807 = F::cast_from(180.0_f64) * t16638;
    let t19809 = t1634 * t637;
    let t19822 = t7035 * t1508;
    let t19823 = F::cast_from(0.51947577317044391276e2_f64) * t19822;
    let t19824 = t496 * t6825;
    let t19825 = F::cast_from(12.0_f64) * t19824;
    let t19843 = F::cast_from(16.0_f64) * t2562 * t500;
    (t19804, t19806, t19807, t19809, t19823, t19825, t19843)
}
