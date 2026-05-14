//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1191/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1191<F: Float>(t3801: F, t6748: F, t1209: F, t6695: F, t460: F, t1214: F, t6587: F, t1211: F, t6744: F, t1277: F, t1294: F, t6573: F, t1774: F, t5245: F, t1210: F, t1215: F, t1295: F, t1775: F, t18037: F, t3561: F, t3567: F, t3572: F, t3732: F, t5225: F, t5237: F, t5251: F, t5417: F, t5429: F, t5498: F, t6580: F, t6745: F) -> (F, F, F, F) {
    let t20692 = t6748 * t3801;
    let t20697 = t1209 * t6695;
    let t20700 = t460 * t6695;
    let t20703 = t6587 * t1214;
    let t20704 = t1211 * t20703;
    let t20709 = t6744 * t1214;
    let t20710 = t1277 * t20709;
    let t20714 = t1277 * t6573 * t1294;
    let t20721 = t1774 * t5245;
    let t20722 = t1211 * t20721;
    let t20727 = t6587 * t1294;
    let t20728 = t1277 * t20727;
    let t20735 = -0.65854491829355115987e0 * t3732 * t6745 - 0.65854491829355115987e0 * t20697 * t1215 - 0.65854491829355115987e0 * t20700 * t1295 + 0.13170898365871023197e1 * t3567 * t20704 - 0.65854491829355115987e0 * t3561 * t6745 + 0.65854491829355115987e0 * t1210 * t20710 - 0.13170898365871023197e1 * t3567 * t20714 - 0.13170898365871023197e1 * t5417 * t5498 - 0.13170898365871023197e1 * t18037 * t1775 + 0.26341796731742046394e1 * t3567 * t20722 + 0.13170898365871023197e1 * t5251 * t5237 + 0.65854491829355115987e0 * t1210 * t20728 + 0.13170898365871023197e1 * t3572 * t6580 + 0.26341796731742046394e1 * t5225 * t5429;
    (t20692, t20703, t20721, t20735)
}
