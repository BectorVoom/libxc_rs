//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 790/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk790<F: Float>(t17895: F, t2394: F, t18011: F, t9609: F, t17903: F, t9524: F, t173: F, t5045: F, t701: F, t5037: F, t5041: F, t3799: F, t3803: F, t227: F, t4995: F, t9: F) -> (F, F, F, F, F, F, F, F, F) {
    let t18015 = t2394 * t17895;
    let t18018 = t9609 * t18011;
    let t18021 = t2394 * t17903;
    let t18024 = t9524 * t18011;
    let t18031 = t173 * t5045;
    let t18032 = t701 * t18031;
    let t18034 = t173 * t5037;
    let t18035 = t701 * t18034;
    let t18037 = t173 * t5041;
    let t18038 = t701 * t18037;
    let t18040 = t3799 * t3803;
    let t18043 = t9 * t227 * t4995;
    (t18015, t18018, t18021, t18024, t18032, t18035, t18038, t18040, t18043)
}
