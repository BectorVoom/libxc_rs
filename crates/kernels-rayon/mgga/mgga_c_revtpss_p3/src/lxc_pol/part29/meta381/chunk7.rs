//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1369/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1369(t1892: f64, t4086: f64, t786: f64, t4104: f64, t2470: f64, t5740: f64, t4101: f64, t1432: f64, t5763: f64, t1385: f64, t5710: f64, t10105: f64, t10109: f64, t10114: f64, t10117: f64, t10120: f64, t10126: f64, t10129: f64, t10137: f64, t10143: f64, t13921: f64, t1399: f64, t1437: f64, t3924: f64, t4118: f64, t5659: f64, t5767: f64, t820: f64) -> f64 {
    let t14238 = t4086 * t1892;
    let t14239 = t786 * t14238;
    let t14241 = 0.19514881078765566038e-1_f64 * t14239 * t4104;
    let t14242 = t5740 * t2470;
    let t14243 = t4101 * t14242;
    let t14252 = t1432 * t5763 * t2470;
    let t14255 = t1385 * t5710;
    let t14266 = -t14241 + 0.13009920719177044025e-1_f64 * t14243 + 0.9757440539382783019e-2_f64 * t10105 + 0.23131639038696784278e-2_f64 * t10109 + t10114 - 0.13170898365871023197e1_f64 * t820 * t4118 * t5659 - t10117 - 0.9757440539382783019e-2_f64 * t10120 - t10126 - t10129 - 0.13009920719177044025e-1_f64 * t14252 + 0.2601984143835408805e-1_f64 * t10137 - 0.13170898365871023197e1_f64 * t820 * t14255 * t1399 - 0.65854491829355115987e0_f64 * t820 * t5767 * t3924 - 0.65854491829355115987e0_f64 * t820 * t1437 * t13921 - 0.23131639038696784278e-2_f64 * t10143;
    t14266
}
