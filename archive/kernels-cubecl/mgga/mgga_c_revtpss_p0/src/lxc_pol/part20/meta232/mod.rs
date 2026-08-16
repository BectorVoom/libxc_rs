//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta232 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1028;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1029;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta232<F: Float>(t2482: F, t27: F, t2719: F, t221: F, t2485: F, t2724: F, t2741: F, t2756: F, t820: F, t843: F, t2726: F, t10665: F, t2723: F, t827: F, t828: F, t821: F, t235: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t10850, t10852, t10853, t10855, t10858, t10859, t10861) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1028::<F>(t2482, t27, t2719, t221, t2485, t2724, t2741, t2756, t820, t843, t2726, t10665, t2723);
        let (t10863, t10866, t10867, t10868) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1029::<F>(t10861, t827, t828, t821, t235);
    (t10850, t10852, t10853, t10855, t10858, t10859, t10861, t10863, t10866, t10867, t10868)
}
