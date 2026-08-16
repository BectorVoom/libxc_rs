//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta556 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2009;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2010;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta556<F: Float>(t92975: F, t243: F, t7021: F, t2732: F, t64: F, t9731: F, t2710: F, t826: F, t10631: F, t10886: F, t7028: F, t159: F, t8779: F, t218: F, t816: F, t10685: F, t1946: F, t10671: F, t7033: F, t25255: F, t2689: F, t10680: F, t1945: F, t807: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t92976, t92979, t92986, t92989, t92991, t92993) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2009::<F>(t92975, t243, t7021, t2732, t64, t9731, t2710, t826, t10631, t10886, t7028, t159, t8779);
        let (t92996, t92998, t93000, t93001, t93004) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2010::<F>(t218, t816, t92993, t10685, t1946, t10671, t7033, t25255, t2689, t10680, t1945, t807);
    (t92976, t92979, t92986, t92989, t92991, t92993, t92996, t92998, t93000, t93001, t93004)
}
