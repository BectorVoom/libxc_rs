//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1103/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1103<F: Float>(t1211: F, t321: F, t43260: F, t15108: F, t47184: F, t52112: F, t14849: F, t804: F, t12276: F, t15102: F, t14843: F, t15097: F, t2053: F, t15081: F, t2376: F, t829: F, t830: F) -> (F, F, F, F, F, F, F, F, F) {
    let t54821 = t321 * t1211;
    let t54823 = 4.0 * t54821 * t43260;
    let t54825 = 2.0 * t321 * t15108;
    let t54832 = 6.0 * t52112 * t47184;
    let t54843 = 6.0 * t804 * t14849;
    let t54852 = 6.0 * t52112 * t12276;
    let t54854 = 2.0 * t321 * t15102;
    let t54866 = 6.0 * t804 * t14843;
    let t54867 = t15097 * t2053;
    let t54880 = t2376 * t15081;
    let t54882 = t829 * t830 * t54880;
    (t54823, t54825, t54832, t54843, t54852, t54854, t54866, t54867, t54882)
}
