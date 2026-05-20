//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2310/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2310<F: Float>(t1225: F, t22671: F, t1012: F, t13006: F, t22688: F, t13027: F, t13020: F, t1774: F, t6628: F) -> (F, F, F, F, F, F, F, F, F) {
    let t24816 = t1225 * t22671;
    let t24817 = t1012 * t24816;
    let t24820 = t13006 * t22688;
    let t24821 = t1012 * t24820;
    let t24826 = t13027 * t22688;
    let t24827 = t1012 * t24826;
    let t24830 = t13020 * t22688;
    let t24831 = t1012 * t24830;
    let t24834 = t1774 * t6628;
    (t24816, t24817, t24820, t24821, t24826, t24827, t24830, t24831, t24834)
}
