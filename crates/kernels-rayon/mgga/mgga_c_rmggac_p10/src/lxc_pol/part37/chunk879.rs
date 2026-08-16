//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 879/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk879(t15078: f64, t5016: f64, t1502: f64, t27: f64, t16129: f64, t69609: f64, t34975: f64, t34976: f64, t665: f64, t9145: f64, t1326: f64, t75307: f64) -> (f64, f64, f64, f64) {
    let t75758 = t5016 * t15078;
    let t75760 = t27 * t1502;
    let t75762 = t69609 * t16129 * t75760;
    let t75767 = 0.1064114997332445985e-4_f64 * t34975 * t34976 * t665 * t9145;
    let t75770 = t1326 * t75307;
    (t75758, t75762, t75767, t75770)
}
