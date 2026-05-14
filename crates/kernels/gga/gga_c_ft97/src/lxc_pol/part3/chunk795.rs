//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 795/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk795<F: Float>(t18109: F, t92: F, t17780: F, t683: F, t3051: F, t458: F, t4974: F, t17732: F, t2404: F, t17727: F, t16579: F, t668: F, t13538: F, t13541: F, t13543: F, t13544: F, t18096: F, t18099: F, t18102: F, t18105: F, t18107: F, t9557: F, t9558: F) -> (F, F, F, F, F, F, F, F) {
    let t18110 = t92 * t18109;
    let t18112 = t683 * t17780;
    let t18113 = t3051 * t18112;
    let t18115 = t458 * t4974;
    let t18117 = t2404 * t17732;
    let t18118 = t92 * t18117;
    let t18120 = t683 * t17727;
    let t18121 = t92 * t18120;
    let t18123 = t668 * t16579;
    let t18124 = t683 * t18123;
    let t18125 = t92 * t18124;
    let t18127 = -t9557 - 4.0 / 27.0 * t9558 - 8.0 / 27.0 * t13538 + t13541 - t13543 - 4.0 / 9.0 * t13544 + 2.0 / 27.0 * t18096 - 10.0 / 27.0 * t18099 + 4.0 / 3.0 * t18102 + 8.0 / 9.0 * t18105 - 2.0 / 9.0 * t18107 - 2.0 * t18110 - 8.0 / 3.0 * t18113 + t18115 / 9.0 - 2.0 / 9.0 * t18118 + 2.0 / 3.0 * t18121 - t18125 / 3.0;
    (t18110, t18113, t18115, t18118, t18121, t18123, t18125, t18127)
}
