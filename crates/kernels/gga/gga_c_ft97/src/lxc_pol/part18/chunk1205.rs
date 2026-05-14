//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1205/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1205<F: Float>(t23057: F, t3266: F, t5674: F, t8411: F, t1871: F, t22952: F, t25883: F, t22953: F, t26016: F, t379: F, t101772: F, t101775: F, t101779: F, t101782: F, t101787: F, t101791: F, t101795: F, t93557: F, t93560: F) -> (F, F, F, F) {
    let t101799 = t5674 * t8411 * t23057 * t3266;
    let t101803 = t22952 * t1871 * t23057 * t25883;
    let t101807 = t22952 * t22953 * t26016 * t379;
    let t101809 = -t101772 + 2.0 / 3.0 * t101775 - t101779 - t101782 - t93557 / 18.0 - 2.0 / 9.0 * t93560 - 4.0 * t101787 + 4.0 / 9.0 * t101791 - t101795 / 8.0 - 2.0 * t101799 - t101803 / 3.0 - t101807 / 18.0;
    (t101799, t101803, t101807, t101809)
}
