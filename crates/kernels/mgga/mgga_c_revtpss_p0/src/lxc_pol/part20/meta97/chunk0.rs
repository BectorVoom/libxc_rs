//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 558/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk558<F: Float>(t225: F, t2760: F, t213: F, t860: F, t256: F, t866: F, t886: F) -> (F, F, F, F, F) {
    let t2761 = t2760 * t225;
    let t2765 = t213 * t860;
    let t2769 = F::cast_from(1.0_f64) / t866 / t256;
    let t2770 = t225 * t2769;
    let t2771 = t886 * t886;
    (t2761, t2765, t2769, t2770, t2771)
}
