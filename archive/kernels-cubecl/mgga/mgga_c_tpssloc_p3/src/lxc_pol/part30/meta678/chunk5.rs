//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2125/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2125<F: Float>(t26003: F, t4028: F, t2314: F, t28864: F, t4034: F, t1873: F, t19289: F, t652: F, t1983: F, t20085: F, t6996: F, t20109: F, t20143: F, t22461: F, t24980: F, t26103: F, t28852: F, t5460: F, t5493: F, t5494: F, t6517: F, t6862: F, t96755: F, t96758: F, t96760: F, t96763: F, t96765: F) -> F {
    let t96767 = F::cast_from(4.0_f64) * t4028 * t26003;
    let t96784 = F::cast_from(2.0_f64) * t2314 * t28864;
    let t96786 = F::cast_from(2.0_f64) * t4034 * t28864;
    let t96789 = F::cast_from(2.0_f64) * t652 * t19289 * t1873;
    let t96792 = F::cast_from(2.0_f64) * t1983 * t6996 * t20085;
    let t96793 = -F::cast_from(2.0_f64) * t5493 * t652 * t6862 - F::cast_from(4.0_f64) * t20109 * t6517 - F::cast_from(2.0_f64) * t20143 * t6517 - F::cast_from(4.0_f64) * t22461 * t5460 - F::cast_from(2.0_f64) * t2314 * t28852 - F::cast_from(4.0_f64) * t24980 * t4028 - F::cast_from(4.0_f64) * t26103 * t5460 - F::cast_from(2.0_f64) * t26103 * t5494 - F::cast_from(2.0_f64) * t28852 * t4034 - t96755 - t96758 + t96760 + t96763 - t96765 - t96767 - t96784 - t96786 - t96789 + t96792;
    t96793
}
