//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta349 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1157;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1158;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta349<F: Float>(t1647: F, t3298: F, t1086: F, t1678: F, t994: F, t12166: F, t378: F, t342: F, t11631: F, t12050: F, t12077: F, t3154: F, t12046: F, t3316: F, t1071: F, t4746: F, t15669: F, t379: F, t1716: F, t2435: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t16509, t16544, t16552, t16553, t16559, t16560) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1157::<F>(t1647, t3298, t1086, t1678, t994, t12166, t378, t342, t11631, t12050, t12077, t3154);
        let (t16566, t16584, t16597, t16600, t16603, t16706) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1158::<F>(t12046, t378, t342, t1647, t3316, t1071, t4746, t15669, t379, t994, t1716, t2435);
    (t16509, t16544, t16552, t16553, t16559, t16560, t16566, t16584, t16597, t16600, t16603, t16706)
}
