//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3791/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3791(t17306: f64, t1811: f64, t1209: f64, t21342: f64, t21333: f64, t487: f64, t1210: f64, t1215: f64, t12603: f64, t1277: f64, t1295: f64, t18054: f64, t18059: f64, t18097: f64, t18103: f64, t1829: f64, t20753: f64, t3567: f64, t3568: f64, t3569: f64, t3737: f64, t3739: f64, t3790: f64, t5237: f64, t5498: f64, t56503: f64, t56508: f64, t6587: f64, t6702: f64, t6745: f64) -> f64 {
    let t72874 = t17306 * t1811;
    let t72877 = t1209 * t21342;
    let t72894 = t21333 * t487;
    let t72899 = 0.65854491829355115987e0_f64 * t1210 * t1277 * t6587 * t3790 - 0.26341796731742046394e1_f64 * t18054 * t5498 + 0.26341796731742046394e1_f64 * t72874 * t3569 - 0.13170898365871023197e1_f64 * t72877 * t1215 + 0.26341796731742046394e1_f64 * t18097 * t5237 - 0.13170898365871023197e1_f64 * t12603 * t6745 - 0.13170898365871023197e1_f64 * t56508 * t1829 + 0.26341796731742046394e1_f64 * t3567 * t3737 * t6702 * t3568 + 0.13170898365871023197e1_f64 * t20753 * t3739 - 0.26341796731742046394e1_f64 * t56503 * t1829 - 0.13170898365871023197e1_f64 * t72894 * t1295 - 0.26341796731742046394e1_f64 * t18059 * t18103;
    t72899
}
