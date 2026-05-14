//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 689/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk689<F: Float>(t1365: F, t30209: F, t6525: F, t9074: F, t9086: F, t9204: F, t29970: F, t4261: F, t29985: F, t30140: F, t12352: F, t2312: F, t12366: F, t484: F, t12427: F, t20883: F, t9079: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t39811 = t6525 * t1365 * t30209;
    let t39849 = t9074 * t9204 * t9086;
    let t39866 = t6525 * t4261 * t29970;
    let t39869 = t9074 * t4261 * t29985;
    let t39893 = t9074 * t1365 * t30140;
    let t39895 = t2312 * t12352;
    let t39897 = t484 * t12366;
    let t39899 = t2312 * t12366;
    let t39901 = t484 * t12427;
    let t39904 = t6525 * t9079 * t20883;
    (t39811, t39849, t39866, t39869, t39893, t39895, t39897, t39899, t39901, t39904)
}
