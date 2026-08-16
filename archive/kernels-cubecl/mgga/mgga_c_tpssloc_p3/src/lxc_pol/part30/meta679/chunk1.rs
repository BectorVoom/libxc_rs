//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2128/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2128<F: Float>(t26161: F, t26162: F, t96830: F, t26114: F, t7468: F, t26179: F, t1266: F, t1980: F, t20098: F, t27996: F, t28811: F, t510: F, t5450: F, t650: F, t652: F, t671: F, t6862: F, t96655: F, t96796: F, t96799: F, t96802: F, t96805: F, t96807: F, t96813: F, t96815: F, t96818: F, t96827: F, t96829: F) -> F {
    let t96833 = F::cast_from(4.0_f64) * t26161 * t26162 * t96830;
    let t96837 = F::cast_from(4.0_f64) * t26114 * t7468;
    let t96839 = F::cast_from(4.0_f64) * t26179 * t7468;
    let t96840 = -F::cast_from(2.0_f64) * t28811 * t652 * t671 - F::cast_from(2.0_f64) * t1266 * t27996 + t1980 * t20098 - t28811 * t650 - F::cast_from(2.0_f64) * t510 * t96655 - t5450 * t6862 + t96796 + t96799 - t96802 + t96805 - t96807 - t96813 - t96815 - t96818 + t96827 - t96829 + t96833 - t96837 - t96839;
    t96840
}
