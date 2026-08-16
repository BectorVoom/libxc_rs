//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta472 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2032;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2033;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta472<F: Float>(t14540: F, t14572: F, t14953: F, t14976: F, t868: F, t4533: F, t72: F, t686: F, t2465: F, t1569: F, t867: F, t786: F, t2467: F, t122: F, t4480: F, t2466: F, t10995: F, t11044: F, t4481: F, t10498: F, t10501: F, t14474: F, t14479: F, t14484: F, t14486: F, t14489: F, t865: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t14978, t14979, t14982, t14983, t14985, t14986, t14987) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2032::<F>(t14540, t14572, t14953, t14976, t868, t4533, t72, t686, t2465, t1569, t867, t786);
        let (t14990, t14991, t14997) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2033::<F>(t14987, t2467, t122, t4480, t2466, t10995, t11044, t4481, t10498, t10501, t14474, t14479, t14484, t14486, t14489, t14979, t14985, t865);
    (t14978, t14979, t14982, t14983, t14986, t14987, t14990, t14991, t14997)
}
