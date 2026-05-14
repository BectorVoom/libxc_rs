//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 825/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk825<F: Float>(t1815: F, t7881: F, t639: F, t1809: F, t7264: F, t2580: F, t5125: F, t1820: F, t1027: F, t1733: F, t1620: F, t1407: F, t2579: F, t1821: F, t1663: F, t995: F) -> (F, F, F, F, F, F) {
    let t7882 = t1815 * t7881;
    let t7884 = 4.0 / 45.0 * t639 * t7882;
    let t7885 = t1809 * t7264;
    let t7887 = 8.0 / 45.0 * t639 * t7885;
    let t7888 = t5125 * t2580;
    let t7890 = 32.0 / 135.0 * t1820 * t7888;
    let t7891 = t1027 * t1733;
    let t7892 = t1809 * t7891;
    let t7894 = 8.0 / 45.0 * t1620 * t7892;
    let t7895 = t2579 * t1407;
    let t7896 = t1821 * t7895;
    let t7898 = 8.0 / 45.0 * t1820 * t7896;
    let t7899 = t995 * t1663;
    (t7884, t7887, t7890, t7894, t7898, t7899)
}
