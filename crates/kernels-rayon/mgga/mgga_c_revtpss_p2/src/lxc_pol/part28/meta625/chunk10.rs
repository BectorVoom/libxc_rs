//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2233/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2233(t1096: f64, t16243: f64, t16352: f64, t1678: f64, t1986: f64, t25476: f64, t25591: f64, t25621: f64, t25625: f64, t25658: f64, t25699: f64, t27426: f64, t27599: f64, t27616: f64, t27661: f64, t27676: f64, t27680: f64, t27687: f64, t3042: f64, t3043: f64, t3271: f64, t5016: f64, t7102: f64, t7145: f64, t7151: f64, t7156: f64, t7160: f64, t7812: f64, t7821: f64, t94095: f64, t988: f64, t999: f64) -> f64 {
    let t100650 = -0.8673628188205199462e0_f64 * t27661 * t25621 + 0.65854491829355115987e0_f64 * t7102 * t16352 + 0.13170898365871023197e1_f64 * t27616 * t3271 + 0.13170898365871023197e1_f64 * t7102 * t16243 + 0.34694512752820797848e1_f64 * t94095 * t27599 - 0.8673628188205199462e0_f64 * t7156 * t27676 - 0.52041769129231196772e1_f64 * t25699 * t7145 * t27687 * t999 + 0.17347256376410398924e1_f64 * t25591 * t7145 * t7821 * t3042 + 0.34694512752820797848e1_f64 * t25591 * t7145 * t27687 * t988 - 0.13170898365871023197e1_f64 * t25658 * t5016 - 0.8673628188205199462e0_f64 * t25625 * t1678 * t1986 - 0.17347256376410398924e1_f64 * t25476 * t27680 - 0.34694512752820797848e1_f64 * t7151 * t7160 * t27687 * t1096 - 0.34694512752820797848e1_f64 * t7151 * t7160 * t27426 * t999 + 0.65854491829355115987e0_f64 * t3043 * t7812;
    t100650
}
