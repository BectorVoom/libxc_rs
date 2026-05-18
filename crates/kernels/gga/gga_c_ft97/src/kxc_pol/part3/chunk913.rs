//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 913/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk913<F: Float>(t18101: F, t92: F, t17753: F, t2404: F, t3051: F, t458: F, t4970: F, t17744: F, t683: F, t17780: F, t4974: F, t17732: F) -> (F, F, F, F, F, F, F) {
    let t18102 = t92 * t18101;
    let t18104 = t2404 * t17753;
    let t18105 = t3051 * t18104;
    let t18107 = t458 * t4970;
    let t18109 = t683 * t17744;
    let t18110 = t92 * t18109;
    let t18112 = t683 * t17780;
    let t18113 = t3051 * t18112;
    let t18115 = t458 * t4974;
    let t18117 = t2404 * t17732;
    (t18102, t18105, t18107, t18110, t18113, t18115, t18117)
}
