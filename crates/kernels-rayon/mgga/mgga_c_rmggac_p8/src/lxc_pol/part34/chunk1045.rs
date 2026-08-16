//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1045/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1045(t78049: f64, t76323: f64, t25820: f64, t77085: f64, t27101: f64, t77088: f64, t352: f64, t76319: f64, t76322: f64, t76326: f64, t77999: f64, t78036: f64, t78038: f64, t78039: f64, t78040: f64, t78046: f64, t78048: f64, t8940: f64) -> f64 {
    let t78050 = 0.8980681276397856423e-1_f64 * t78049;
    let t78051 = 0.14967802127329760705e-1_f64 * t76323;
    let t78052 = t25820 * t77085;
    let t78053 = 0.8980681276397856423e-1_f64 * t78052;
    let t78054 = t27101 * t77088;
    let t78055 = 0.5987120850931904282e-1_f64 * t78054;
    let t78056 = -t78036 - t78038 + t78039 + t78040 + t76319 + t76322 + 0.11974241701863808564e0_f64 * t8940 * t77999 * t352 + t78046 - t78048 - t78050 - t78051 + t76326 + t78053 + t78055;
    t78056
}
