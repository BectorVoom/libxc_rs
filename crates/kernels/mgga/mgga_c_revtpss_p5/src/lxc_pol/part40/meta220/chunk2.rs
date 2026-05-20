//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 876/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk876<F: Float>(t225: F, t385: F, t4930: F, t1678: F, t342: F, t1695: F, t999: F, t1079: F, t1096: F, t3269: F, t1086: F, t1647: F) -> (F, F, F, F, F) {
    let t4932 = t4930 * t225 * t385;
    let t4935 = t342 * t1678;
    let t4940 = t1695 * t999;
    let t4941 = t1079 * t4940;
    let t4946 = t1695 * t1096;
    let t4947 = t3269 * t4946;
    let t4954 = t1647 * t1086;
    (t4932, t4935, t4941, t4947, t4954)
}
