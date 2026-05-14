//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 791/1363 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk791<F: Float>(t1277: F, t5497: F, t1204: F, t1210: F, t1215: F, t1271: F, t1274: F, t1295: F, t1770: F, t1775: F, t1813: F, t1829: F, t3556: F, t3561: F, t3567: F, t3572: F, t3732: F, t460: F, t495: F, t5216: F, t5220: F, t5225: F, t5231: F, t5237: F, t5246: F, t5251: F, t5414: F, t5417: F, t5423: F, t5429: F) -> (F, F) {
    let t5498 = t1277 * t5497;
    let t5501 = 0.65854491829355115987e0 * t5216 * t495 - 0.65854491829355115987e0 * t5220 * t1215 + 0.65854491829355115987e0 * t1770 * t1271 - 0.65854491829355115987e0 * t5225 * t1295 - 0.65854491829355115987e0 * t3556 * t1775 + 0.13170898365871023197e1 * t3567 * t5231 - 0.65854491829355115987e0 * t3572 * t1775 + 0.65854491829355115987e0 * t1210 * t5237 - 0.65854491829355115987e0 * t1210 * t5246 + 0.65854491829355115987e0 * t1204 * t1813 - 0.65854491829355115987e0 * t5251 * t1215 + 0.65854491829355115987e0 * t460 * t5414 - 0.65854491829355115987e0 * t5417 * t1295 - 0.65854491829355115987e0 * t3561 * t1829 + 0.65854491829355115987e0 * t1210 * t5423 - 0.65854491829355115987e0 * t3732 * t1829 + 0.13170898365871023197e1 * t1274 * t5429 - 0.65854491829355115987e0 * t1274 * t5498;
    (t5498, t5501)
}
