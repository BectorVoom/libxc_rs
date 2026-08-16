//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta260 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1399;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1400;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta260<F: Float>(t10190: F, t2990: F, t2986: F, t2770: F, t607: F, t2250: F, t4510: F, t2980: F, t9288: F, t977: F, t9258: F, t978: F, t3008: F, t343: F, t984: F, t4546: F, t271: F, t2775: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t10191, t10192, t10195) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1399::<F>(t10190, t2990, t2986, t2770, t607, t2250);
        let (t10196, t10199, t10200, t10203, t10204, t10208, t10209, t10213) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1400::<F>(t10195, t4510, t2980, t9288, t977, t9258, t978, t3008, t343, t984, t4546, t271, t2775);
    (t10191, t10192, t10195, t10196, t10199, t10200, t10203, t10204, t10208, t10209, t10213)
}
