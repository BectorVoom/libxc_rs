//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 982/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk982<F: Float>(t13857: F, t9793: F, t221: F, t5627: F, t9921: F, t3978: F, t2619: F, t5635: F, t1398: F, t1882: F, t13848: F, t3938: F, t9818: F, t9816: F, t125: F, t5658: F) -> (F, F, F, F, F, F) {
    let t13858 = t9793 * t13857;
    let t13877 = t221 * t5627;
    let t13878 = t9921 * t13877;
    let t13880 = 0.50820002809285328225e-3 * t3978 * t13878;
    let t13887 = t5635 * t2619;
    let t13926 = t1882 * t1398;
    let t13941 = t9818 * t13848 * t3938;
    let t13943 = 0.10164000561857065645e-3 * t9816 * t13941;
    let t13944 = t125 * t5658;
    (t13858, t13880, t13887, t13926, t13943, t13944)
}
