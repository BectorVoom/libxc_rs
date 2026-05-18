//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1250/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1250<F: Float>(t15113: F, t321: F, t14854: F, t804: F, t1211: F, t43260: F, t15108: F, t47184: F, t52112: F, t14849: F, t12276: F, t15102: F) -> (F, F, F, F, F, F, F, F) {
    let t54809 = F::new(2.0) * t321 * t15113;
    let t54811 = F::new(6.0) * t804 * t14854;
    let t54821 = t321 * t1211;
    let t54823 = F::new(4.0) * t54821 * t43260;
    let t54825 = F::new(2.0) * t321 * t15108;
    let t54832 = F::new(6.0) * t52112 * t47184;
    let t54843 = F::new(6.0) * t804 * t14849;
    let t54852 = F::new(6.0) * t52112 * t12276;
    let t54854 = F::new(2.0) * t321 * t15102;
    (t54809, t54811, t54823, t54825, t54832, t54843, t54852, t54854)
}
