//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 907/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk907<F: Float>(t4939: F, t688: F, t200: F, t807: F, t17895: F, t2394: F, t9609: F, t17903: F, t9524: F, t173: F, t5045: F, t701: F) -> (F, F, F, F, F, F, F) {
    let t18010 = t4939 * t688;
    let t18011 = t18010 * t200;
    let t18012 = t807 * t18011;
    let t18015 = t2394 * t17895;
    let t18018 = t9609 * t18011;
    let t18021 = t2394 * t17903;
    let t18024 = t9524 * t18011;
    let t18031 = t173 * t5045;
    let t18032 = t701 * t18031;
    (t18010, t18012, t18015, t18018, t18021, t18024, t18032)
}
