//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 999/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk999<F: Float>(t1797: F, t2020: F, t5586: F, t570: F, t2060: F, t6313: F, t7815: F, t6319: F, t507: F, t8630: F, t142: F, t35364: F, t6375: F, t6293: F, t8888: F, t30120: F, t9649: F) -> (F, F, F, F, F, F, F, F) {
    let t39615 = t2020 * t1797;
    let t39617 = t570 * t5586;
    let t39620 = t2060 * t7815 * t6313;
    let t39623 = t2060 * t7815 * t6319;
    let t39626 = t2060 * t507 * t8630;
    let t39629 = t35364 * t142 * t6375;
    let t39632 = t8888 * t142 * t6293;
    let t39640 = t30120 * t9649;
    (t39615, t39617, t39620, t39623, t39626, t39629, t39632, t39640)
}
