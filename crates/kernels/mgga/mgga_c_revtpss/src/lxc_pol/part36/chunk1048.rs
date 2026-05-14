//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1048/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1048<F: Float>(t27883: F, t786: F, t7286: F, t213: F, t7910: F, t1885: F, t26024: F, t25972: F, t5622: F, t1889: F, t25978: F, t25986: F, t5609: F, t2661: F, t13846: F, t1941: F) -> (F, F, F, F, F, F, F, F, F) {
    let t27899 = t786 * t27883;
    let t27900 = t27899 * t7286;
    let t27909 = t213 * t7910;
    let t27921 = t26024 * t1885;
    let t27924 = t25972 * t5622;
    let t27926 = t25978 * t1889;
    let t27928 = t25986 * t5609;
    let t27929 = t2661 * t27928;
    let t27932 = t1941 * t13846;
    (t27899, t27900, t27909, t27921, t27924, t27926, t27928, t27929, t27932)
}
