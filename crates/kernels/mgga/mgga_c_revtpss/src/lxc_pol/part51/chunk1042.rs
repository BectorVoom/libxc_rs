//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1042/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1042<F: Float>(t125401: F, t125440: F, t125481: F, t125526: F, t125943: F, t127333: F, t127367: F, t127409: F, t1921: F, t8602: F, t2045: F, t7939: F, t119422: F, t119424: F, t121458: F, t121460: F, t125208: F, t1464: F, t28235: F, t3: F, t32343: F, t33984: F, t575: F, t5808: F, t8603: F) -> (F, F) {
    let t127412 = t125401 + t125440 + t125481 + t125526 + t125943 + t127333 + t127367 + t127409;
    let t127416 = t8602 * t1921;
    let t127421 = t7939 * t2045;
    let t127425 = t127412 * t3 * t575 + t1464 * t33984 + t1921 * t32343 + 2.0 * t2045 * t28235 + t5808 * t8603 + t119422 + t119424 + 2.0 * t121458 + 2.0 * t121460 + t125208 + t127416 + 2.0 * t127421;
    (t127412, t127425)
}
