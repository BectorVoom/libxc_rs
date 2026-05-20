//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3791/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3791<F: Float>(t17306: F, t1811: F, t1209: F, t21342: F, t21333: F, t487: F, t1210: F, t1215: F, t12603: F, t1277: F, t1295: F, t18054: F, t18059: F, t18097: F, t18103: F, t1829: F, t20753: F, t3567: F, t3568: F, t3569: F, t3737: F, t3739: F, t3790: F, t5237: F, t5498: F, t56503: F, t56508: F, t6587: F, t6702: F, t6745: F) -> F {
    let t72874 = t17306 * t1811;
    let t72877 = t1209 * t21342;
    let t72894 = t21333 * t487;
    let t72899 = F::cast_from(0.65854491829355115987e0_f64) * t1210 * t1277 * t6587 * t3790 - F::cast_from(0.26341796731742046394e1_f64) * t18054 * t5498 + F::cast_from(0.26341796731742046394e1_f64) * t72874 * t3569 - F::cast_from(0.13170898365871023197e1_f64) * t72877 * t1215 + F::cast_from(0.26341796731742046394e1_f64) * t18097 * t5237 - F::cast_from(0.13170898365871023197e1_f64) * t12603 * t6745 - F::cast_from(0.13170898365871023197e1_f64) * t56508 * t1829 + F::cast_from(0.26341796731742046394e1_f64) * t3567 * t3737 * t6702 * t3568 + F::cast_from(0.13170898365871023197e1_f64) * t20753 * t3739 - F::cast_from(0.26341796731742046394e1_f64) * t56503 * t1829 - F::cast_from(0.13170898365871023197e1_f64) * t72894 * t1295 - F::cast_from(0.26341796731742046394e1_f64) * t18059 * t18103;
    t72899
}
