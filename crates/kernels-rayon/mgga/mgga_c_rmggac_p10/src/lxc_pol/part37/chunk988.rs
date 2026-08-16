//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 988/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk988(t305: f64, t77871: f64, t14516: f64, t8537: f64, t2471: f64, t838: f64, t2141: f64, t326: f64, t9530: f64, t2147: f64, t76197: f64, t76199: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t77872 = t305 * t77871;
    let t77873 = 0.14967802127329760705e-1_f64 * t77872;
    let t77874 = t14516 * t8537;
    let t77875 = 0.27274661654245341728e-1_f64 * t77874;
    let t77876 = t838 * t2471;
    let t77877 = t77876 * t2141;
    let t77878 = 0.13637330827122670864e-1_f64 * t77877;
    let t77879 = t326 * t9530;
    let t77880 = t77879 * t2147;
    let t77881 = 0.68186654135613354322e-2_f64 * t77880;
    let t77883 = 0.17961362552795712846e0_f64 * t76197;
    let t77884 = 0.44903406381989282115e-1_f64 * t76199;
    (t77873, t77875, t77878, t77881, t77883, t77884)
}
