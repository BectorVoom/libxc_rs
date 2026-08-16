//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 777/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk777(t9074: f64, t9086: f64, t9204: f64, t29970: f64, t4261: f64, t6525: f64, t29985: f64, t1365: f64, t30140: f64, t12352: f64, t2312: f64, t12366: f64, t484: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t39849 = t9074 * t9204 * t9086;
    let t39866 = t6525 * t4261 * t29970;
    let t39869 = t9074 * t4261 * t29985;
    let t39893 = t9074 * t1365 * t30140;
    let t39895 = t2312 * t12352;
    let t39897 = t484 * t12366;
    (t39849, t39866, t39869, t39893, t39895, t39897)
}
