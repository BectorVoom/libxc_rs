//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta423 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1596;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta423<F: Float>(t1659: F, t3230: F, t1660: F, t3201: F, t1058: F, t4798: F, t1053: F, t4797: F, t15127: F, t15125: F, t15191: F, t11134: F, t11136: F, t11138: F, t11140: F, t11890: F, t15132: F, t15137: F, t15142: F, t15147: F, t15151: F, t15156: F, t15160: F, t15189: F, t15195: F) -> (F, F, F, F, F) {
        let (t15859, t15862, t15865, t15866, t15885) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1596::<F>(t1659, t3230, t1660, t3201, t1058, t4798, t1053, t4797, t15127, t15125, t15191, t11134, t11136, t11138, t11140, t11890, t15132, t15137, t15142, t15147, t15151, t15156, t15160, t15189, t15195);
    (t15859, t15862, t15865, t15866, t15885)
}
