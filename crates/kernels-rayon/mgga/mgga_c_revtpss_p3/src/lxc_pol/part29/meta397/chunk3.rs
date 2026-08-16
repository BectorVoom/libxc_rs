//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1424/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1424(t2470: f64, t4522: f64, t874: f64, t10657: f64, t10916: f64, t10921: f64, t14577: f64, t14581: f64, t14590: f64, t14596: f64, t14603: f64, t14608: f64, t14663: f64, t14939: f64, t14948: f64, t1559: f64, t213: f64, t234: f64, t2754: f64, t2815: f64, t4424: f64, t4494: f64, t4514: f64, t820: f64, t879: f64) -> f64 {
    let t14951 = t874 * t4522 * t2470;
    let t14953 = -t14577 - 0.65854491829355115987e0_f64 * t4514 * t4494 * t2754 + 0.73171657588172351096e-2_f64 * t14581 - 0.13170898365871023197e1_f64 * t820 * t2815 * t4424 - t14590 + 0.23131639038696784278e-2_f64 * t10916 + 0.54878743191129263322e-2_f64 * t10921 + t14596 + 0.39029762157531132075e-1_f64 * t14603 - t14608 - 0.65854491829355115987e0_f64 * t820 * t879 * t14663 + 0.65854491829355115987e0_f64 * t213 * t234 * t14939 - 0.65854491829355115987e0_f64 * t820 * t10657 * t1559 + 0.11565819519348392139e-2_f64 * t14948 - 0.13009920719177044025e-1_f64 * t14951;
    t14953
}
