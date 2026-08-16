//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 648/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk648(t325: f64, t5058: f64, t128: f64, t25640: f64, t305: f64, t4616: f64, t326: f64, t793: f64, t874: f64) -> (f64, f64, f64, f64, f64) {
    let t26857 = t5058 * t325;
    let t27041 = t25640 * t128;
    let t27048 = t305 * t4616;
    let t27055 = t326 * t4616;
    let t27101 = t793 * t874;
    (t26857, t27041, t27048, t27055, t27101)
}
