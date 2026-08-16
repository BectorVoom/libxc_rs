//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1031/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1031(t2141: f64, t77876: f64, t326: f64, t9530: f64, t2147: f64, t69146: f64, t76180: f64, t76182: f64, t76184: f64, t77863: f64, t77864: f64, t77868: f64, t77869: f64, t77870: f64, t77873: f64, t77875: f64) -> f64 {
    let t77877 = t77876 * t2141;
    let t77878 = 0.13637330827122670864e-1_f64 * t77877;
    let t77879 = t326 * t9530;
    let t77880 = t77879 * t2147;
    let t77881 = 0.68186654135613354322e-2_f64 * t77880;
    let t77882 = t77863 + t77864 + 0.93188427318671584245e-2_f64 * t76180 - 0.15531404553111930708e-1_f64 * t76182 - 0.6212561821244772283e-2_f64 * t76184 + t77868 - t77869 - t77870 - t77873 - t69146 - t77875 - t77878 - t77881;
    t77882
}
