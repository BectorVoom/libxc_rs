//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1453/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1453(t22369: f64, t2782: f64, t22005: f64, t4086: f64, t543: f64, t6888: f64, t72: f64, t1432: f64, t686: f64, t10049: f64, t10117: f64, t10126: f64, t10129: f64, t10137: f64, t10143: f64, t1399: f64, t14252: f64, t1437: f64, t22009: f64, t22253: f64, t22353: f64, t22362: f64, t22366: f64, t5659: f64, t5735: f64, t5755: f64, t6862: f64, t820: f64) -> f64 {
    let t22370 = t2782 * t22369;
    let t22373 = t4086 * t22005 * t543;
    let t22374 = t2782 * t22373;
    let t22379 = t6888 * t72;
    let t22381 = t1432 * t22379 * t686;
    let t22384 = -0.65854491829355115987e0_f64 * t5755 * t22009 * t1399 - 0.13170898365871023197e1_f64 * t5755 * t5735 * t5659 - t10117 - 0.54878743191129263322e-2_f64 * t22353 + 0.13170898365871023197e1_f64 * t820 * t10049 * t6862 - t10126 - t10129 - 0.26019841438354088051e-1_f64 * t14252 + 0.13009920719177044025e-1_f64 * t10137 - 0.10975748638225852664e-1_f64 * t22362 + 0.10975748638225852664e-1_f64 * t22366 + 0.54878743191129263322e-2_f64 * t22370 + 0.54878743191129263322e-2_f64 * t22374 - 0.65854491829355115987e0_f64 * t820 * t1437 * t22253 + 0.9757440539382783019e-2_f64 * t22381 - 0.11565819519348392139e-2_f64 * t10143;
    t22384
}
