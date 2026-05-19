//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 736/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk736<F: Float>(t1960: F, t880: F, t309: F, t314: F, t180: F, t621: F, t609: F, t851: F, t323: F, t1219: F, t2130: F, t615: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7921 = F::cast_from(0.65854491829355115987e0_f64) * t1960 * t880;
    let t7922 = t309 * t309;
    let t7923 = t7922 * t314;
    let t7924 = t7923 * t180;
    let t7926 = F::cast_from(0.8673628188205199462e0_f64) * t7924 * t621;
    let t7927 = t851 * t609;
    let t7929 = F::cast_from(0.13170898365871023197e1_f64) * t7927 * t323;
    let t7930 = t2130 * t1219;
    let t7931 = t615 * t7930;
    (t7921, t7922, t7923, t7924, t7926, t7927, t7929, t7930, t7931)
}
