//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 822/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk822<F: Float>(t203: F, t7829: F, t184: F, t221: F, t1406: F, t181: F, t997: F, t562: F, t577: F, t5379: F, t1045: F, t1672: F, t211: F, t2826: F, t612: F, t1006: F, t1868: F) -> (F, F, F, F, F, F, F) {
    let t7830 = t203 * t7829;
    let t7831 = t7830 * t184;
    let t7833 = 2.0 / 15.0 * t7831 * t221;
    let t7834 = t1406 * t181;
    let t7835 = t7834 * t184;
    let t7837 = 4.0 / 15.0 * t7835 * t997;
    let t7838 = t562 * t577;
    let t7839 = t7838 * t184;
    let t7841 = 8.0 / 15.0 * t7839 * t997;
    let t7843 = 4.0 / 15.0 * t5379 * t997;
    let t7844 = t1672 * t1045;
    let t7845 = t211 * t7844;
    let t7846 = 4.0 / 135.0 * t7845;
    let t7848 = 4.0 / 15.0 * t2826 * t612;
    let t7850 = 2.0 / 15.0 * t1006 * t1868;
    (t7833, t7837, t7841, t7843, t7846, t7848, t7850)
}
