//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 597/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk597<F: Float>(t1277: F, t3790: F, t1204: F, t1210: F, t1215: F, t1271: F, t1274: F, t1295: F, t3552: F, t3556: F, t3561: F, t3567: F, t3569: F, t3572: F, t3576: F, t3585: F, t3729: F, t3732: F, t3739: F, t460: F, t495: F) -> (F, F) {
    let t3791 = t1277 * t3790;
    let t3794 = F::cast_from(0.65854491829355115987e0_f64) * t3552 * t495 - F::cast_from(0.13170898365871023197e1_f64) * t3556 * t1215 + F::cast_from(0.13170898365871023197e1_f64) * t1204 * t1271 - F::cast_from(0.13170898365871023197e1_f64) * t3561 * t1295 + F::cast_from(0.13170898365871023197e1_f64) * t3567 * t3569 - F::cast_from(0.13170898365871023197e1_f64) * t3572 * t1215 + F::cast_from(0.13170898365871023197e1_f64) * t1210 * t3576 - F::cast_from(0.65854491829355115987e0_f64) * t1210 * t3585 + F::cast_from(0.65854491829355115987e0_f64) * t460 * t3729 - F::cast_from(0.13170898365871023197e1_f64) * t3732 * t1295 + F::cast_from(0.13170898365871023197e1_f64) * t1274 * t3739 - F::cast_from(0.65854491829355115987e0_f64) * t1274 * t3791;
    (t3791, t3794)
}
