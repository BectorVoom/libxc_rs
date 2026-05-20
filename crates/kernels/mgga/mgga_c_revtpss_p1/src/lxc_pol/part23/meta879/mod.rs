//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta879 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2786;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2787;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta879<F: Float>(t14114: F, t14216: F, t14145: F, t2482: F, t4114: F, t6843: F, t1432: F, t22379: F, t2470: F, t1437: F, t4104: F, t6861: F, t22307: F, t686: F, t72: F, t1385: F, t136: F, t2457: F, t3964: F, t6888: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t74862, t74866, t74873, t74880) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2786::<F>(t14114, t14216, t14145, t2482, t4114, t6843, t1432, t22379, t2470, t1437, t4104, t6861);
        let (t74884, t74886, t74892, t74893, t74901) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2787::<F>(t1432, t22307, t686, t72, t1385, t1437, t2482, t6843, t4104, t136, t2457, t3964, t6888);
    (t74862, t74866, t74873, t74880, t74884, t74886, t74892, t74893, t74901)
}
