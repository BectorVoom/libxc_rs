//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 946/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk946<F: Float>(t1131: F, t3821: F, t2574: F, t265: F, t10002: F, t5064: F, t242: F, t1882: F, t5070: F, t5181: F, t684: F, t724: F) -> (F, F, F, F, F) {
    let t18622 = t1131 * t3821;
    let t18624 = t2574 * t265 * t18622;
    let t18627 = t10002 * t5064;
    let t18628 = t242 * t18627;
    let t18633 = t1882 * t5070;
    let t18636 = t724 * t5181 * t684;
    (t18624, t18627, t18628, t18633, t18636)
}
