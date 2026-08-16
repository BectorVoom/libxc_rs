//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta711 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2468;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2469;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta711<F: Float>(t47944: F, t14078: F, t2470: F, t3915: F, t13735: F, t2435: F, t10115: F, t1900: F, t14189: F, t22: F, t46389: F, t543: F, t5735: F, t1432: F, t5763: F, t9288: F, t10069: F, t14124: F, t14129: F, t14231: F, t10139: F, t136: F, t2457: F, t5659: F, t14202: F, t9303: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t47945, t47948, t47953, t47961, t47964, t47967) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2468::<F>(t47944, t14078, t2470, t3915, t13735, t2435, t10115, t1900, t14189, t22, t46389, t543, t5735);
        let (t47971, t47979, t47981, t47985, t48004, t48005) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2469::<F>(t1432, t5763, t9288, t10069, t14124, t14129, t14231, t10139, t136, t2457, t5659, t14202, t9303);
    (t47945, t47948, t47953, t47961, t47964, t47967, t47971, t47979, t47981, t47985, t48004, t48005)
}
