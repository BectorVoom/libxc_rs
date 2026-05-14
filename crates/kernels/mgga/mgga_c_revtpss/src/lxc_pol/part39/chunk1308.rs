//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1308/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1308<F: Float>(t1210: F, t1215: F, t12603: F, t1295: F, t18043: F, t18047: F, t18054: F, t18059: F, t18062: F, t18065: F, t18070: F, t18073: F, t1813: F, t1829: F, t3552: F, t3556: F, t3567: F, t3569: F, t3572: F, t3585: F, t5220: F, t5246: F, t5251: F, t5423: F) -> (F,) {
    let t18080 = 0.13170898365871023197e1 * t1210 * t18043 - 0.13170898365871023197e1 * t1210 * t18047 + 0.65854491829355115987e0 * t3552 * t1813 + 0.13170898365871023197e1 * t3556 * t5423 - 0.13170898365871023197e1 * t18054 * t1295 - 0.65854491829355115987e0 * t5220 * t3585 + 0.13170898365871023197e1 * t18059 * t3569 - 0.13170898365871023197e1 * t18062 * t1215 - 0.13170898365871023197e1 * t18065 * t1295 - 0.13170898365871023197e1 * t3572 * t5246 + 0.26341796731742046394e1 * t3567 * t18070 + 0.13170898365871023197e1 * t3567 * t18073 - 0.13170898365871023197e1 * t12603 * t1829 - 0.65854491829355115987e0 * t5251 * t3585;
    (t18080,)
}
