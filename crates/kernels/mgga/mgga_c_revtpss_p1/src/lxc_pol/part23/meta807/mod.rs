//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta807 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2640;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2641;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta807<F: Float>(t2801: F, t62967: F, t14563: F, t14568: F, t14598: F, t14600: F, t4423: F, t676: F, t14602: F, t2482: F, t2811: F, t6016: F, t10535: F, t136: F, t2457: F, t6017: F, t10542: F, t18726: F, t2439: F, t2440: F, t6072: F, t2444: F, t689: F, t15003: F, t51258: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t62968, t62983, t62987, t62992) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2640::<F>(t2801, t62967, t14563, t14568, t14598, t14600, t4423, t676, t14602, t2482, t2811, t6016);
        let (t62999, t63015, t63050, t63053, t63058) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2641::<F>(t10535, t136, t2457, t6017, t10542, t18726, t2439, t2440, t6072, t2444, t689, t15003, t51258);
    (t62968, t62983, t62987, t62992, t62999, t63015, t63050, t63053, t63058)
}
