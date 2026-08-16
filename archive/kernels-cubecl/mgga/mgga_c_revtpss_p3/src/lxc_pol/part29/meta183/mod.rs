//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta183 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk860;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta183<F: Float>(t3937: F, t3938: F, t3936: F, t159: F, t550: F, t216: F, t124: F, t3829: F, t800: F, t1376: F, t2689: F, t1353: F, t1413: F) -> (F, F, F, F, F, F) {
        let (t3940, t3943, t3944, t3946, t3950, t3951) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk860::<F>(t3937, t3938, t3936, t159, t550, t216, t124, t3829, t800, t1376, t2689, t1353, t1413);
    (t3940, t3943, t3944, t3946, t3950, t3951)
}
