//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 851/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk851<F: Float>(t2147: F, t463: F, t7885: F, t8336: F, t2219: F, t848: F, t2132: F, t322: F, t7896: F, t8103: F, t2225: F, t879: F, t8099: F, t2230: F, t30009: F, t3915: F, t8347: F) -> (F, F, F, F, F, F, F) {
    let t33000 = t7885 * t2147 * t8336 * t463;
    let t33008 = t848 * t2219;
    let t33015 = 0.52041769129231196772e1 * t7896 * t2132 * t8103 * t322;
    let t33019 = 0.52041769129231196772e1 * t7896 * t2132 * t2225 * t879;
    let t33028 = t7896 * t2132 * t8099 * t322;
    let t33031 = 0.52041769129231196772e1 * t30009 * t2230;
    let t33037 = 0.39512695097613069591e1 * t8347 * t3915;
    (t33000, t33008, t33015, t33019, t33028, t33031, t33037)
}
