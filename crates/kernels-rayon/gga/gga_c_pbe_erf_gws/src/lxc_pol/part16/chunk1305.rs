//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1305/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1305(t1172: f64, t1211: f64, t318: f64, t15113: f64, t321: f64, t14854: f64, t804: f64, t43260: f64, t15108: f64, t1167: f64, t13756: f64, t14368: f64, t30098: f64, t4062: f64, t4066: f64, t4120: f64, t52090: f64, t52092: f64, t52094: f64, t52763: f64, t52791: f64, t52829: f64, t8804: f64, t9740: f64) -> f64 {
    let t54802 = t1172 * t318 * t1211;
    let t54809 = 2.0_f64 * t321 * t15113;
    let t54811 = 6.0_f64 * t804 * t14854;
    let t54821 = t321 * t1211;
    let t54823 = 4.0_f64 * t54821 * t43260;
    let t54825 = 2.0_f64 * t321 * t15108;
    let t54829 = -t1167 * t4062 * t52094 + 12.0_f64 * t13756 * t4066 * t8804 - 6.0_f64 * t13756 * t4120 * t52791 + 2.0_f64 * t14368 * t4062 * t52763 + 4.0_f64 * t14368 * t4062 * t52829 - t4062 * t4120 * t9740 - 12.0_f64 * t30098 * t54802 + t52090 + 3.0_f64 * t52092 - t54809 + t54811 + t54823 - t54825;
    t54829
}
