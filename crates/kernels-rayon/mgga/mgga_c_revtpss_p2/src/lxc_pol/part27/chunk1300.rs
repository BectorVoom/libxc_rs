//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1300/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1300(t5: f64, t96748: f64, t96779: f64, t96803: f64, t96830: f64, t117: f64, t10259: f64, t2371: f64, t27060: f64, t29432: f64, t670: f64, t7586: f64, t94956: f64, t94958: f64, t94960: f64, t94962: f64, t94964: f64, t94966: f64, t94968: f64, t94970: f64, t94972: f64, t94993: f64, t96706: f64, t96709: f64) -> (f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t96833 = piecewise3(t8, 0.0_f64, t96748 + t96779 + t96803 + t96830);
    let t96834 = t96833 * t117;
    let t96835 = 2.0_f64 * t10259 * t7586 + 6.0_f64 * t2371 * t27060 + 6.0_f64 * t2371 * t29432 + 6.0_f64 * t670 * t96706 + t94956 + t94958 + t94960 + t94962 + t94964 + t94966 + t94968 + t94970 + t94972 + t94993 + 6.0_f64 * t96709 + t96834;
    (t96834, t96835)
}
