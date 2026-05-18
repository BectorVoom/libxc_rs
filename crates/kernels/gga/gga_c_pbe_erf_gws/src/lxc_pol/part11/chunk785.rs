//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 785/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk785<F: Float>(t1046: F, t3479: F, t10969: F, t997: F, t3351: F, t7651: F, t1809: F, t1620: F, t1044: F, t3469: F, t1815: F, t639: F) -> (F, F, F, F, F, F, F, F) {
    let t12744 = F::new(2.0) / F::new(5.0) * t3479 * t1046;
    let t12746 = F::new(4.0) / F::new(5.0) * t10969 * t997;
    let t12747 = t7651 * t3351;
    let t12748 = t1809 * t12747;
    let t12750 = F::new(16.0) / F::new(15.0) * t1620 * t12748;
    let t12751 = t3469 * t1044;
    let t12752 = t1815 * t12751;
    let t12754 = F::new(8.0) / F::new(15.0) * t639 * t12752;
    (t12744, t12746, t12747, t12748, t12750, t12751, t12752, t12754)
}
