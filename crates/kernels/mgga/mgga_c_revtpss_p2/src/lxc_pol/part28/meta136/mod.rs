//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta136 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk749;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk750;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta136<F: Float>(t2988: F, t3014: F, t2868: F, t2871: F, t2878: F, t2921: F, t2929: F, t2935: F, t2938: F, t2943: F, t2945: F, t2963: F, t2968: F, t2971: F, t2980: F, t2982: F, t2987: F, t2989: F, t3007: F, t3012: F, t311: F, t946: F, t955: F, t965: F, t974: F, t300: F, t960: F) -> (F, F, F, F) {
        let (t3015, t3018) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk749::<F>(t2988, t3014, t2868, t2871, t2878, t2921, t2929, t2935, t2938, t2943, t2945, t2963, t2968, t2971, t2980, t2982, t2987, t2989, t3007, t3012, t311, t946, t955, t965, t974);
        let (t3019, t3021, t3022) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk750::<F>(t300, t3018, t2980, t960);
    (t3015, t3019, t3021, t3022)
}
