//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 774/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk774<F: Float>(t17732: F, t2594: F, t446: F, t1131: F, t3746: F, t2354: F, t3281: F, t4969: F, t713: F, t4965: F, t505: F) -> (F, F, F, F, F, F) {
    let t17733 = t2594 * t17732;
    let t17734 = t446 * t17733;
    let t17736 = t3746 * t1131;
    let t17737 = t2354 * t17736;
    let t17738 = t3281 * t17737;
    let t17740 = t4969 * t713;
    let t17741 = t2354 * t17740;
    let t17742 = t446 * t17741;
    let t17744 = t4965 * t505;
    (t17734, t17736, t17738, t17740, t17742, t17744)
}
