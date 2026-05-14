//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1205/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1205<F: Float>(t1466: F, t28869: F, t681: F, t458: F, t6962: F, t6219: F, t10688: F, t28930: F, t2843: F, t4299: F, t6386: F, t1501: F, t15073: F, t29416: F, t6213: F, t1477: F, t193: F, t24964: F, t25395: F, t25400: F, t25413: F, t25459: F, t25467: F, t25480: F, t28835: F, t28868: F, t28870: F, t28993: F, t29008: F, t3704: F, t6210: F, t6972: F) -> (F, F, F, F) {
    let t112565 = 2.0 / 9.0 * t1466 * t681 * t28869;
    let t112566 = t6962 * t458;
    let t112568 = t112566 * t6219 / 27.0;
    let t112579 = t10688 * t28930;
    let t112582 = t2843 * t6386 * t4299;
    let t112585 = t2843 * t1501 * t15073;
    let t112602 = t29416 * t6213 / 9.0;
    let t112603 = t112565 + t112568 - 2.0 / 3.0 * t1466 * t193 * t28835 * t25395 - t1466 * t193 * t28835 * t25400 / 3.0 - t25480 * t6972 / 3.0 + 8.0 * t112579 + 8.0 * t112582 + 4.0 * t112585 + t1466 * t3704 * t1477 * t25413 / 9.0 - 2.0 / 3.0 * t6210 * t28870 - 2.0 / 3.0 * t1466 * t193 * t24964 * t28868 - t29008 * t25467 / 9.0 - t25459 * t28993 / 9.0 - t112602;
    (t112579, t112582, t112585, t112603)
}
