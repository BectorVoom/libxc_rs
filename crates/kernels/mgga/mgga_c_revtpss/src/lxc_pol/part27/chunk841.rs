//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 841/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk841<F: Float>(t1398: F, t215: F, t268: F, t543: F, t4101: F, t2453: F, t4100: F, t281: F, t68: F, t10080: F, t10082: F, t10085: F, t10090: F, t10098: F, t10102: F, t10105: F, t10109: F, t10114: F, t10117: F, t10120: F, t10126: F, t10129: F, t10130: F, t1399: F, t4057: F, t4114: F, t4118: F, t5755: F, t820: F, t9912: F, t9995: F) -> F {
    let t10136 = t268 * t215 * t1398 * t543;
    let t10137 = t4101 * t10136;
    let t10139 = t2453 * t4100;
    let t10142 = t281 * t68 * t1398 * t543;
    let t10143 = t10139 * t10142;
    let t10145 = F::new(0.32927245914677557992e-1) * t10080 + F::new(0.16463622957338778996e-1) * t10085 - F::new(0.19756347548806534796e1) * t820 * t4118 * t4057 - F::new(0.39512695097613069591e1) * t820 * t10090 * t9995 + F::new(0.39512695097613069591e1) * t820 * t4114 * t9912 - F::new(0.39029762157531132076e-1) * t10098 + t10102 + F::new(0.29272321618148349057e-1) * t10105 + F::new(0.34697458558045176417e-2) * t10109 + t10114 - t10117 - F::new(0.29272321618148349057e-1) * t10120 - F::new(0.19756347548806534796e1) * t5755 * t10082 * t1399 - t10126 - t10129 - F::new(0.19756347548806534796e1) * t820 * t10130 * t1399 + F::new(0.39029762157531132076e-1) * t10137 - F::new(0.34697458558045176417e-2) * t10143;
    t10145
}
