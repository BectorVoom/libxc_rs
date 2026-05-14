//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1372/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1372<F: Float>(t6649: F, t8232: F, t23571: F, t50249: F, t50235: F, t5855: F, t12602: F, t23997: F, t26857: F, t8392: F, t1882: F, t26890: F, t104469: F, t105937: F, t12277: F, t12338: F, t12710: F, t13204: F, t144: F, t167: F, t2185: F, t2230: F, t446: F, t47659: F, t574: F, t5947: F, t6630: F, t95720: F, t95723: F, t95725: F, t95730: F, t95842: F) -> (F, F, F, F) {
    let t106798 = t8232 * t6649;
    let t106803 = t50249 * t23571;
    let t106807 = t50235 * t5855;
    let t106811 = t23997 * t12602;
    let t106830 = 2.0 / 27.0 * t8392 * t26857;
    let t106837 = 4.0 / 9.0 * t1882 * t26890;
    let t106838 = -4.0 / 81.0 * t106798 + 4.0 / 9.0 * t47659 * t95842 * t13204 + 4.0 / 9.0 * t47659 * t106803 * t12710 + 8.0 / 9.0 * t47659 * t106807 * t12338 + 4.0 / 3.0 * t446 * t144 * t106811 + 2.0 / 9.0 * t95720 + 4.0 / 3.0 * t446 * t144 * t104469 - t95723 / 9.0 + 2.0 / 27.0 * t95725 + 2.0 / 3.0 * t446 * t574 * t12277 * t5947 + 2.0 / 3.0 * t446 * t2185 * t167 * t105937 - t106830 - 2.0 / 9.0 * t95730 + 2.0 / 3.0 * t446 * t2185 * t2230 * t6630 - t106837;
    (t106803, t106807, t106811, t106838)
}
