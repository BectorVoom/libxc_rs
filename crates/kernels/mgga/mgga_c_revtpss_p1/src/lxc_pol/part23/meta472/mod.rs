//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta472 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1922;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1923;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta472<F: Float>(t3154: F, t4866: F, t4893: F, t3117: F, t11922: F, t6272: F, t3115: F, t1668: F, t3181: F, t372: F, t1045: F, t4574: F, t12131: F, t6266: F, t15691: F, t1011: F, t1068: F, t15689: F, t15700: F, t19951: F, t19954: F, t19957: F, t19960: F, t19963: F, t19968: F, t3106: F, t4892: F, t6331: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t19971, t19972, t19973, t19976, t19977, t19979, t19980, t19981) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1922::<F>(t3154, t4866, t4893, t3117, t11922, t6272, t3115, t1668, t3181, t372, t1045, t4574);
        let (t19982, t19985, t19986, t19989) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1923::<F>(t19980, t19981, t12131, t6266, t15691, t1011, t1068, t15689, t15700, t19951, t19954, t19957, t19960, t19963, t19968, t19973, t19977, t3106, t4892, t6331);
    (t19971, t19972, t19973, t19976, t19977, t19979, t19981, t19982, t19985, t19986, t19989)
}
