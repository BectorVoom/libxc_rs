//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta577 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1986;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta577<F: Float>(t5876: F, t670: F, t1448: F, t6836: F, t6816: F, t1868: F, t5778: F, t10309: F, t607: F, t843: F, t1962: F, t41154: F) -> (F, F, F, F, F, F, F) {
        let (t85360, t86753, t86771, t86815, t92568, t92612, t92742) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1986::<F>(t5876, t670, t1448, t6836, t6816, t1868, t5778, t10309, t607, t843, t1962, t41154);
    (t85360, t86753, t86771, t86815, t92568, t92612, t92742)
}
