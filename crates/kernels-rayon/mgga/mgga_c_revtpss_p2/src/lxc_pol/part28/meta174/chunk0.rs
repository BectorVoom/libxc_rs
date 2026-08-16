//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 885/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk885(t1277: f64, t3790: f64, t1204: f64, t1210: f64, t1215: f64, t1271: f64, t1274: f64, t1295: f64, t3552: f64, t3556: f64, t3561: f64, t3567: f64, t3569: f64, t3572: f64, t3576: f64, t3585: f64, t3729: f64, t3732: f64, t3739: f64, t460: f64, t495: f64) -> (f64, f64) {
    let t3791 = t1277 * t3790;
    let t3794 = 0.65854491829355115987e0_f64 * t3552 * t495 - 0.13170898365871023197e1_f64 * t3556 * t1215 + 0.13170898365871023197e1_f64 * t1204 * t1271 - 0.13170898365871023197e1_f64 * t3561 * t1295 + 0.13170898365871023197e1_f64 * t3567 * t3569 - 0.13170898365871023197e1_f64 * t3572 * t1215 + 0.13170898365871023197e1_f64 * t1210 * t3576 - 0.65854491829355115987e0_f64 * t1210 * t3585 + 0.65854491829355115987e0_f64 * t460 * t3729 - 0.13170898365871023197e1_f64 * t3732 * t1295 + 0.13170898365871023197e1_f64 * t1274 * t3739 - 0.65854491829355115987e0_f64 * t1274 * t3791;
    (t3791, t3794)
}
