//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1284/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1284<F: Float>(t1882: F, t31312: F, t110401: F, t111070: F, t111109: F, t111111: F, t111121: F, t123883: F, t13885: F, t14182: F, t14196: F, t18206: F, t18507: F, t18514: F, t18617: F, t18671: F, t1901: F, t24668: F, t24793: F, t28140: F, t28294: F, t28344: F, t3972: F, t446: F, t51609: F, t6075: F, t6161: F, t65592: F, t67847: F, t6837: F, t6917: F, t729: F, t762: F) -> (F,) {
    let t124753 = t1882 * t31312;
    let t124777 = -t111070 + 2.0 * t1901 * t28140 * t6161 * t18206 + 4.0 / 3.0 * t1901 * t13885 * t24668 * t18617 - 4.0 / 3.0 * t1901 * t110401 * t18671 + 2.0 / 3.0 * t1901 * t14182 * t28344 * t18514 + 2.0 / 9.0 * t124753 - 2.0 / 9.0 * t1901 * t24793 * t18507 - 4.0 / 3.0 * t1901 * t67847 * t28294 + t1901 * t65592 * t6075 / 9.0 + 2.0 / 9.0 * t1901 * t51609 * t6917 + 8.0 / 27.0 * t111109 - 8.0 / 27.0 * t111111 - t111121 - 4.0 / 9.0 * t1901 * t14196 * t123883 + 2.0 / 3.0 * t446 * t729 * t762 * t6837 * t3972;
    (t124777,)
}
