//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 989/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk989(t11577: f64, t11609: f64, t11614: f64, t11860: f64, t354: f64, t1039: f64, t3461: f64, t1010: f64, t11033: f64, t11036: f64, t2381: f64, t2391: f64, t3358: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11862 = t11577 + t11609 + t11614 + t11860;
    let t11863 = t354 * t11862;
    let t11864 = t1039 * t3461;
    let t11866 = t11033 * t1010;
    let t11868 = t11036 * t2381;
    let t11870 = t3358 * t2391;
    (t11862, t11863, t11864, t11866, t11868, t11870)
}
