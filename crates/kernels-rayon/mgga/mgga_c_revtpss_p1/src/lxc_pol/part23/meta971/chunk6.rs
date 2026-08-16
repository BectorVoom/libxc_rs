//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3285/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3285(t14100: f64, t22399: f64, t1904: f64, t213: f64, t22390: f64, t225: f64, t47504: f64, t47512: f64, t47886: f64, t47899: f64, t47904: f64, t561: f64, t5728: f64, t73666: f64, t73671: f64, t73673: f64, t73676: f64, t73705: f64, t73707: f64, t74802: f64, t85509: f64, t86280: f64) -> f64 {
    let t86285 = t14100 * t22399;
    let t86291 = -0.7805952431506226415e-1_f64 * t73666 + 0.98781737744032673976e-1_f64 * t73671 + t47504 - 0.19756347548806534796e1_f64 * t74802 * t1904 - 0.21951497276451705328e-1_f64 * t73673 - 0.65854491829355115984e-1_f64 * t73676 + 0.39512695097613069592e1_f64 * t22390 * t5728 - 0.54878743191129263322e-2_f64 * t85509 - 0.11044544084478153697e-3_f64 * t47512 + 0.65854491829355115987e0_f64 * t213 * t86280 * t225 * t561 - 0.29272321618148349057e-1_f64 * t86285 - t47886 - 0.39029762157531132076e-2_f64 * t47899 - 0.91069445034239308177e-1_f64 * t47904 + 0.16463622957338778996e-1_f64 * t73705 + 0.43902994552903410656e-1_f64 * t73707;
    t86291
}
