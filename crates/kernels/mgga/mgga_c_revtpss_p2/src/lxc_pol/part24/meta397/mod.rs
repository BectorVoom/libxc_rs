//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta397 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1323;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1324;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta397<F: Float>(t2495: F, t9385: F, t2491: F, t744: F, t760: F, t2492: F, t2514: F, t9367: F, t9371: F, t200: F, t631: F, t202: F, t635: F, t2548: F, t2490: F, t2595: F, t39490: F, t39492: F, t39495: F, t39498: F, t39501: F, t39506: F, t39508: F, t39510: F, t39512: F, t39515: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t39815, t39816, t39818, t39821, t39823, t39825, t39840) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1323::<F>(t2495, t9385, t2491, t744, t760, t2492, t2514, t9367, t9371, t200, t631, t202, t635);
        let (t39871, t39875, t39886, t39894, t39909) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1324::<F>(t2514, t2492, t2548, t2490, t2595, t39490, t39492, t39495, t39498, t39501, t39506, t39508, t39510, t39512, t39515);
    (t39815, t39816, t39818, t39821, t39823, t39825, t39840, t39871, t39875, t39886, t39894, t39909)
}
