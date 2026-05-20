//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta324 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1334;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1335;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta324<F: Float>(t2737: F, t9802: F, t221: F, t2485: F, t2754: F, t2484: F, t2749: F, t836: F, t853: F, t2662: F, t2661: F, t2646: F, t2482: F, t596: F, t823: F, t2487: F, t27: F, t2719: F, t2724: F, t2741: F, t2756: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t10826, t10832, t10833, t10836, t10838, t10841) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1334::<F>(t2737, t9802, t221, t2485, t2754, t2484, t2749, t836, t853, t2662, t2661, t2646);
        let (t10842, t10845, t10846, t10852, t10853, t10855) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1335::<F>(t10841, t2484, t2482, t596, t823, t2487, t27, t2719, t221, t2485, t2724, t2741, t2756);
    (t10826, t10832, t10833, t10836, t10838, t10841, t10842, t10845, t10846, t10852, t10853, t10855)
}
