//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1110/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1110<F: Float>(t2339: F, t624: F, t10208: F, t68: F, t10368: F, t55: F, t45972: F, t7565: F, t12627: F, t2142: F, t42859: F, t487: F, t1276: F, t2148: F, t13038: F, t26894: F, t26921: F) -> (F, F, F, F, F, F, F, F) {
    let t94978 = t624 * t2339;
    let t94982 = t68 * t10208;
    let t96733 = t55 * t10368;
    let t96804 = t45972 * t7565;
    let t96861 = t12627 * t2142;
    let t96886 = t487 * t42859;
    let t96888 = t2148 * t96886 * t1276;
    let t96889 = t13038 * t2142;
    let t96927 = t26894 * t26921;
    (t94978, t94982, t96733, t96804, t96861, t96888, t96889, t96927)
}
