//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 956/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk956(t1427: f64, t6918: f64, t1424: f64, t1904: f64, t213: f64, t3894: f64, t3898: f64, t3910: f64, t3922: f64, t5601: f64, t5604: f64, t561: f64, t5715: f64, t5719: f64, t5723: f64, t6889: f64, t6896: f64) -> (f64, f64) {
    let t6919 = t1427 * t6918;
    let t6922 = t3894 - t3898 - 0.10975748638225852664e-1_f64 * t5601 + 0.10975748638225852664e-1_f64 * t5719 + t3910 + 0.19514881078765566038e-1_f64 * t5604 - 0.19514881078765566038e-1_f64 * t5723 - t3922 + 0.65854491829355115987e0_f64 * t213 * t6889 * t561 - 0.13170898365871023197e1_f64 * t5715 * t1904 + 0.13170898365871023197e1_f64 * t1424 * t6896 - 0.65854491829355115987e0_f64 * t1424 * t6919;
    (t6919, t6922)
}
