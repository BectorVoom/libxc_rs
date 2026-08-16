//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 609/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk609(t1277: f64, t5497: f64, t1204: f64, t1210: f64, t1215: f64, t1271: f64, t1274: f64, t1295: f64, t1770: f64, t1775: f64, t1813: f64, t1829: f64, t3556: f64, t3561: f64, t3567: f64, t3572: f64, t3732: f64, t460: f64, t495: f64, t5216: f64, t5220: f64, t5225: f64, t5231: f64, t5237: f64, t5246: f64, t5251: f64, t5414: f64, t5417: f64, t5423: f64, t5429: f64) -> f64 {
    let t5498 = t1277 * t5497;
    let t5501 = 0.65854491829355115987e0_f64 * t5216 * t495 - 0.65854491829355115987e0_f64 * t5220 * t1215 + 0.65854491829355115987e0_f64 * t1770 * t1271 - 0.65854491829355115987e0_f64 * t5225 * t1295 - 0.65854491829355115987e0_f64 * t3556 * t1775 + 0.13170898365871023197e1_f64 * t3567 * t5231 - 0.65854491829355115987e0_f64 * t3572 * t1775 + 0.65854491829355115987e0_f64 * t1210 * t5237 - 0.65854491829355115987e0_f64 * t1210 * t5246 + 0.65854491829355115987e0_f64 * t1204 * t1813 - 0.65854491829355115987e0_f64 * t5251 * t1215 + 0.65854491829355115987e0_f64 * t460 * t5414 - 0.65854491829355115987e0_f64 * t5417 * t1295 - 0.65854491829355115987e0_f64 * t3561 * t1829 + 0.65854491829355115987e0_f64 * t1210 * t5423 - 0.65854491829355115987e0_f64 * t3732 * t1829 + 0.13170898365871023197e1_f64 * t1274 * t5429 - 0.65854491829355115987e0_f64 * t1274 * t5498;
    t5501
}
