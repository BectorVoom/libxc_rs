//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1137/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1137<F: Float>(t1172: F, t1211: F, t318: F, t15113: F, t321: F, t14854: F, t804: F, t43260: F, t15108: F, t1167: F, t13756: F, t14368: F, t30098: F, t4062: F, t4066: F, t4120: F, t52090: F, t52092: F, t52094: F, t52763: F, t52791: F, t52829: F, t8804: F, t9740: F) -> (F,) {
    let t54802 = t1172 * t318 * t1211;
    let t54809 = 2.0 * t321 * t15113;
    let t54811 = 6.0 * t804 * t14854;
    let t54821 = t321 * t1211;
    let t54823 = 4.0 * t54821 * t43260;
    let t54825 = 2.0 * t321 * t15108;
    let t54829 = -t1167 * t4062 * t52094 + 12.0 * t13756 * t4066 * t8804 - 6.0 * t13756 * t4120 * t52791 + 2.0 * t14368 * t4062 * t52763 + 4.0 * t14368 * t4062 * t52829 - t4062 * t4120 * t9740 - 12.0 * t30098 * t54802 + t52090 + 3.0 * t52092 - t54809 + t54811 + t54823 - t54825;
    (t54829,)
}
