//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1072/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1072(t1599: f64, t18222: f64, t17259: f64, t17267: f64, t17274: f64, t17276: f64, t2093: f64, t4413: f64, t1591: f64, t6136: f64, t2118: f64, t4479: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t18223 = t1599 * t18222;
    let t18244 = 0.23214722222222222222e-2_f64 * t17259;
    let t18246 = 0.25794135802469135802e-2_f64 * t17267;
    let t18248 = 0.30952962962962962962e-2_f64 * t17274;
    let t18249 = 0.10317654320987654321e-2_f64 * t17276;
    let t18253 = t2093 * t4413;
    let t18256 = t6136 * t1591;
    let t18268 = t2118 * t4479;
    (t18223, t18244, t18246, t18248, t18249, t18253, t18256, t18268)
}
