//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 617/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk617(t15130: f64, t2471: f64, t326: f64, t650: f64, t15132: f64, t15134: f64, t15138: f64, t118: f64, t15530: f64, t15164: f64, t15167: f64, t15170: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15546 = 0.68186654135613354325e-2_f64 * t15130;
    let t15547 = t326 * t2471;
    let t15548 = t15547 * t650;
    let t15549 = 0.34093327067806677161e-2_f64 * t15548;
    let t15550 = 0.20455996240684006296e-1_f64 * t15132;
    let t15551 = 0.40911992481368012592e-1_f64 * t15134;
    let t15552 = 0.10227998120342003148e-1_f64 * t15138;
    let t15557 = 0.39914139006212695214e-1_f64 * t118 * t15530;
    let t15559 = 0.20455996240684006298e-1_f64 * t15164;
    let t15560 = 0.2727466165424534173e-1_f64 * t15167;
    let t15561 = 0.13637330827122670865e-1_f64 * t15170;
    (t15546, t15547, t15549, t15550, t15551, t15552, t15557, t15559, t15560, t15561)
}
