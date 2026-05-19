//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1420/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1420<F: Float>(t18108: F, t3737: F, t17288: F, t487: F, t1204: F, t1210: F, t1215: F, t12666: F, t12673: F, t1274: F, t1295: F, t1770: F, t1775: F, t18084: F, t18087: F, t18090: F, t18097: F, t18103: F, t1829: F, t3556: F, t3567: F, t3729: F, t3732: F, t3791: F, t5225: F, t5237: F, t5414: F, t5417: F, t5498: F) -> F {
    let t18109 = t3737 * t18108;
    let t18114 = t17288 * t487;
    let t18121 = -F::cast_from(0.65854491829355115987e0_f64) * t12666 * t1775 + F::cast_from(0.65854491829355115987e0_f64) * t1210 * t18084 - F::cast_from(0.13170898365871023197e1_f64) * t18087 * t1295 - F::cast_from(0.65854491829355115987e0_f64) * t1210 * t18090 + F::cast_from(0.13170898365871023197e1_f64) * t3556 * t5237 - F::cast_from(0.65854491829355115987e0_f64) * t5417 * t3791 - F::cast_from(0.13170898365871023197e1_f64) * t18097 * t1215 + F::cast_from(0.65854491829355115987e0_f64) * t1770 * t3729 - F::cast_from(0.13170898365871023197e1_f64) * t3567 * t18103 - F::cast_from(0.65854491829355115987e0_f64) * t5225 * t3791 + F::cast_from(0.26341796731742046394e1_f64) * t1274 * t18109 + F::cast_from(0.13170898365871023197e1_f64) * t1204 * t5414 - F::cast_from(0.13170898365871023197e1_f64) * t18114 * t1215 - F::cast_from(0.65854491829355115987e0_f64) * t12673 * t1829 - F::cast_from(0.13170898365871023197e1_f64) * t3732 * t5498;
    t18121
}
