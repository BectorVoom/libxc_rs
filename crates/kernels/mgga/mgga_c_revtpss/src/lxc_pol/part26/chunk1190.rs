//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1190/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1190<F: Float>(t2471: F, t26563: F, t10985: F, t26576: F, t2062: F, t2769: F, t786: F, t10997: F, t26519: F, t93157: F, t2453: F, t2458: F, t7399: F) -> (F, F, F, F, F) {
    let t95927 = t26563 * t2471;
    let t95930 = F::cast_from(0.46263278077393568556e-2_f64) * t26576 * t10985;
    let t95936 = t786 * t2062 * t2769;
    let t95937 = t95936 * t10997;
    let t95945 = t93157 * t26519;
    let t95948 = t2453 * t7399 * t2458;
    (t95927, t95930, t95937, t95945, t95948)
}
