//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1420/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1420(t18108: f64, t3737: f64, t17288: f64, t487: f64, t1204: f64, t1210: f64, t1215: f64, t12666: f64, t12673: f64, t1274: f64, t1295: f64, t1770: f64, t1775: f64, t18084: f64, t18087: f64, t18090: f64, t18097: f64, t18103: f64, t1829: f64, t3556: f64, t3567: f64, t3729: f64, t3732: f64, t3791: f64, t5225: f64, t5237: f64, t5414: f64, t5417: f64, t5498: f64) -> f64 {
    let t18109 = t3737 * t18108;
    let t18114 = t17288 * t487;
    let t18121 = -0.65854491829355115987e0_f64 * t12666 * t1775 + 0.65854491829355115987e0_f64 * t1210 * t18084 - 0.13170898365871023197e1_f64 * t18087 * t1295 - 0.65854491829355115987e0_f64 * t1210 * t18090 + 0.13170898365871023197e1_f64 * t3556 * t5237 - 0.65854491829355115987e0_f64 * t5417 * t3791 - 0.13170898365871023197e1_f64 * t18097 * t1215 + 0.65854491829355115987e0_f64 * t1770 * t3729 - 0.13170898365871023197e1_f64 * t3567 * t18103 - 0.65854491829355115987e0_f64 * t5225 * t3791 + 0.26341796731742046394e1_f64 * t1274 * t18109 + 0.13170898365871023197e1_f64 * t1204 * t5414 - 0.13170898365871023197e1_f64 * t18114 * t1215 - 0.65854491829355115987e0_f64 * t12673 * t1829 - 0.13170898365871023197e1_f64 * t3732 * t5498;
    t18121
}
