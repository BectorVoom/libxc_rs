//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 648/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk648<F: Float>(t3014: F, t972: F, t3093: F, t357: F, t1065: F, t2857: F, t2852: F, t3181: F, t1062: F, t3204: F, t3147: F, t72: F, t3088: F, t3299: F, t1043: F, t3154: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4733 = t3014 * t972;
    let t4786 = t3093 * t357;
    let t4801 = t1065 * t2857;
    let t4806 = t3181 * t2852;
    let t4837 = t3204 * t1062;
    let t4890 = t3147 * t72;
    let t4891 = t3088 * t4890;
    let t4892 = t3299 * t4891;
    let t4894 = t3154 * t1043;
    (t4733, t4786, t4801, t4806, t4837, t4890, t4891, t4892, t4894)
}
