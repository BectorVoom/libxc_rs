//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1624/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1624<F: Float>(t1392: F, t73: F, t1412: F, t5591: F, t1398: F, t1882: F, t13848: F, t3938: F, t9818: F, t9816: F, t125: F, t5658: F) -> (F, F, F, F, F, F) {
    let t13902 = t1392 * t73;
    let t13910 = t1412 * t5591;
    let t13926 = t1882 * t1398;
    let t13941 = t9818 * t13848 * t3938;
    let t13943 = F::cast_from(0.10164000561857065645e-3_f64) * t9816 * t13941;
    let t13944 = t125 * t5658;
    (t13902, t13910, t13926, t13941, t13943, t13944)
}
