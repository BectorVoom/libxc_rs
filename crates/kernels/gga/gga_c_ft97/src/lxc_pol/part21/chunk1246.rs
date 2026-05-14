//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1246/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1246<F: Float>(t422: F, t4698: F, t100634: F, t104695: F, t104704: F, t104920: F, t118942: F, t118970: F, t119012: F, t119133: F, t119137: F, t1355: F, t23715: F, t23810: F, t26721: F, t30067: F, t30084: F, t30091: F, t3052: F, t379: F, t40087: F, t40223: F, t40234: F, t5570: F, t5797: F, t925: F) -> (F,) {
    let t119157 = t422 * t4698;
    let t119170 = -0.10947790369858991997e1 * t23810 * t119137 + 0.22653425206514361674e0 * t1355 * t119133 + 0.45306850413028723348e0 * t30067 * t5797 - 0.66678001092592592595e-1 * t104704 * t30084 - 0.66678001092592592595e-1 * t23715 * t5570 * t104695 * t925 - 0.13335600218518518519e0 * t23715 * t100634 * t26721 * t3052 - 0.33339000546296296297e-1 * t23715 * t5570 * t119157 * t379 + 0.90613700826057446696e0 * t104920 * t30091 - 0.4379116147943596799e1 * t40087 * t118942 + 0.90613700826057446696e0 * t40223 * t118970 - 0.13592055123908617004e1 * t40234 * t119012;
    (t119170,)
}
