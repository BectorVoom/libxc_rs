//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 795/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk795<F: Float>(t225: F, t9849: F, t9850: F, t9852: F, t9869: F, t4010: F, t73: F, t9400: F, t3889: F, t9737: F, t1394: F, t9628: F, t1392: F, t1395: F, t4045: F, t4050: F, t4053: F, t539: F, t541: F, t5650: F) -> (F,) {
    let t9872 = (t9849 + t9850 + t9852 + t9869) * t225;
    let t9880 = t73 * t4010;
    let t9881 = t9880 * t9400;
    let t9884 = t9737 * t3889;
    let t9887 = t1394 * t9628;
    let t9890 = -36.0 * t1392 * t4050 + 9.0 * t1392 * t4053 + 9.0 * t1395 * t4045 + 60.0 * t539 * t9881 + 3.0 * t539 * t9887 - t541 * t9872 - 36.0 * t5650 * t9884;
    (t9890,)
}
