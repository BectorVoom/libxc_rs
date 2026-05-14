//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 717/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk717<F: Float>(t1053: F, t2075: F, t574: F, t605: F, t12600: F, t144: F, t1060: F, t1651: F, t569: F, t1643: F, t2205: F, t2230: F, t925: F, t12711: F, t12716: F, t12720: F, t12726: F, t12730: F, t12734: F, t12739: F, t12743: F, t12748: F, t12752: F, t1901: F, t446: F) -> (F,) {
    let t12754 = t1053 * t2075;
    let t12756 = t574 * t605 * t12754;
    let t12759 = t144 * t12600;
    let t12763 = t569 * t1060 * t1651;
    let t12767 = t2205 * t1060 * t1643;
    let t12771 = t569 * t2230 * t925;
    let t12774 = -4.0 / 9.0 * t1901 * t12711 + 4.0 / 27.0 * t1901 * t12716 - 2.0 / 27.0 * t1901 * t12720 - 10.0 / 81.0 * t1901 * t12726 + t1901 * t12730 / 9.0 + 2.0 / 27.0 * t1901 * t12734 + t1901 * t12739 / 9.0 + 2.0 / 9.0 * t1901 * t12743 + 4.0 / 9.0 * t1901 * t12748 + 4.0 / 27.0 * t12752 + t446 * t12756 / 3.0 + 2.0 / 3.0 * t446 * t12759 - t446 * t12763 / 9.0 - 2.0 / 27.0 * t446 * t12767 - t446 * t12771 / 9.0;
    (t12774,)
}
