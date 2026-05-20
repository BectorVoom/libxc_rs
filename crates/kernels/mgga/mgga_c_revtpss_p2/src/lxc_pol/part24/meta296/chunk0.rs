//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1081/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1081<F: Float>(t20850: F, t480: F, t12552: F, t6518: F, t3520: F, t6534: F, t5265: F, t5274: F, t12916: F, t6689: F, t3718: F, t11249: F, t6628: F) -> (F, F, F, F, F, F, F) {
    let t20851 = t20850 * t480;
    let t20890 = t12552 * t6518;
    let t20895 = t3520 * t6534;
    let t20917 = t5274 * t5265;
    let t20926 = t12916 * t6689;
    let t20927 = t3718 * t20926;
    let t20956 = t6628 * t11249;
    (t20851, t20890, t20895, t20917, t20926, t20927, t20956)
}
