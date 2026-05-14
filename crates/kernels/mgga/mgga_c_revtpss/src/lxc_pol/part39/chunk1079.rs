//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1079/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1079<F: Float>(t4057: F, t5673: F, t5674: F, t13848: F, t3938: F, t9818: F, t9816: F, t125: F, t5658: F, t1399: F, t2689: F, t5618: F, t1413: F, t5591: F, t547: F, t807: F) -> (F, F, F, F, F, F) {
    let t13937 = t5673 * t5674 * t4057;
    let t13941 = t9818 * t13848 * t3938;
    let t13943 = 0.10164000561857065645e-3 * t9816 * t13941;
    let t13944 = t125 * t5658;
    let t13946 = t5673 * t13944 * t1399;
    let t13949 = t2689 * t5618;
    let t13951 = t1413 * t5591;
    let t13952 = t547 * t13951;
    let t13954 = 0.57165357490759649296e-4 * t807 * t13952;
    (t13937, t13943, t13944, t13946, t13949, t13954)
}
