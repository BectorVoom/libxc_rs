//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 649/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk649(t838: f64, t874: f64, t107: f64, t1539: f64, t1454: f64, t201: f64, t837: f64, t235: f64, t325: f64, t6477: f64, t875: f64, t899: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t27176 = t838 * t874;
    let t28317 = t1539 * t107;
    let t29122 = t1454 * t201;
    let t29837 = t837 * t874;
    let t29838 = t235 * t29837;
    let t30080 = t6477 * t325;
    let t30204 = t899 * t875;
    (t27176, t28317, t29122, t29837, t29838, t30080, t30204)
}
