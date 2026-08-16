//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1030/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1030(t76175: f64, t76178: f64, t76186: f64, t76188: f64, t76190: f64, t36: f64, t9565: f64, t305: f64, t14516: f64, t8537: f64, t2471: f64, t838: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t77863 = 0.40911992481368012592e-1_f64 * t76175;
    let t77864 = 0.20455996240684006296e-1_f64 * t76178;
    let t77868 = 0.20455996240684006298e-1_f64 * t76186;
    let t77869 = 0.2727466165424534173e-1_f64 * t76188;
    let t77870 = 0.13637330827122670865e-1_f64 * t76190;
    let t77871 = t9565 * t36;
    let t77872 = t305 * t77871;
    let t77873 = 0.14967802127329760705e-1_f64 * t77872;
    let t77874 = t14516 * t8537;
    let t77875 = 0.27274661654245341728e-1_f64 * t77874;
    let t77876 = t838 * t2471;
    (t77863, t77864, t77868, t77869, t77870, t77871, t77873, t77875, t77876)
}
