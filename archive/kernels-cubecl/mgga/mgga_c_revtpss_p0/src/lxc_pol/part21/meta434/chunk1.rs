//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1943/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1943<F: Float>(t4057: F, t5673: F, t5674: F, t13848: F, t3938: F, t9818: F, t9816: F, t125: F, t5658: F) -> (F, F, F, F) {
    let t13937 = t5673 * t5674 * t4057;
    let t13941 = t9818 * t13848 * t3938;
    let t13943 = F::cast_from(0.10164000561857065645e-3_f64) * t9816 * t13941;
    let t13944 = t125 * t5658;
    (t13937, t13941, t13943, t13944)
}
