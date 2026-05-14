//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 962/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk962<F: Float>(t377: F, t4206: F, t180: F, t5079: F, t1160: F, t1539: F, t1639: F, t980: F, t5319: F, t310: F, t5300: F, t4147: F, t4180: F, t16986: F, t4200: F, t12309: F, t4242: F) -> (F, F, F, F, F, F, F, F) {
    let t18910 = t377 * t4206;
    let t18912 = t180 * t5079;
    let t18914 = t1160 * t18912 * t1539;
    let t18916 = t980 * t1639;
    let t18918 = t377 * t5319;
    let t18920 = t310 * t5300;
    let t18925 = t4180 * t4147;
    let t18930 = t16986 * t4200;
    let t18935 = t12309 * t4242;
    (t18910, t18914, t18916, t18918, t18920, t18925, t18930, t18935)
}
