//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta408 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1419;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta408<F: Float>(t18950: F, t923: F, t18909: F, t2908: F, t141: F, t18913: F, t11341: F, t18904: F, t18926: F, t930: F, t18930: F, t15169: F, t15170: F, t15189: F, t15192: F, t15198: F, t18944: F) -> (F, F, F, F, F, F, F) {
        let (t18951, t18961, t18964, t18967, t18970, t18973, t18977) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1419::<F>(t18950, t923, t18909, t2908, t141, t18913, t11341, t18904, t18926, t930, t18930, t15169, t15170, t15189, t15192, t15198, t18944);
    (t18951, t18961, t18964, t18967, t18970, t18973, t18977)
}
