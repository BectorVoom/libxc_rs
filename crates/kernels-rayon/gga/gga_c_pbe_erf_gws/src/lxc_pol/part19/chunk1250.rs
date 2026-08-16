//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1250/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1250(t15113: f64, t321: f64, t14854: f64, t804: f64, t1211: f64, t43260: f64, t15108: f64, t47184: f64, t52112: f64, t14849: f64, t12276: f64, t15102: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t54809 = 2.0_f64 * t321 * t15113;
    let t54811 = 6.0_f64 * t804 * t14854;
    let t54821 = t321 * t1211;
    let t54823 = 4.0_f64 * t54821 * t43260;
    let t54825 = 2.0_f64 * t321 * t15108;
    let t54832 = 6.0_f64 * t52112 * t47184;
    let t54843 = 6.0_f64 * t804 * t14849;
    let t54852 = 6.0_f64 * t52112 * t12276;
    let t54854 = 2.0_f64 * t321 * t15102;
    (t54809, t54811, t54823, t54825, t54832, t54843, t54852, t54854)
}
