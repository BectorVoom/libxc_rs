//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta896 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2855;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta896<F: Float>(t23121: F, t40188: F, t40121: F, t40132: F, t40139: F, t40088: F, t40099: F, t40103: F, t40115: F, t40131: F, t40137: F, t50048: F, t76986: F, t76987: F, t76988: F, t76991: F, t76992: F, t76995: F) -> (F, F, F, F, F) {
        let (t76997, t76998, t76999, t77000, t77001) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2855::<F>(t23121, t40188, t40121, t40132, t40139, t40088, t40099, t40103, t40115, t40131, t40137, t50048, t76986, t76987, t76988, t76991, t76992, t76995);
    (t76997, t76998, t76999, t77000, t77001)
}
