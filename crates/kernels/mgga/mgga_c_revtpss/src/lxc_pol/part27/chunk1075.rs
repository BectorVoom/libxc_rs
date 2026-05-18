//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1075/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1075<F: Float>(t13180: F, t225: F, t1294: F, t3738: F, t1204: F, t1210: F, t1215: F, t12666: F, t12673: F, t12690: F, t12696: F, t1271: F, t1274: F, t1295: F, t13166: F, t13170: F, t13174: F, t13177: F, t3552: F, t3556: F, t3561: F, t3585: F, t3729: F, t3732: F, t3739: F, t3791: F, t460: F, t495: F) -> (F, F, F, F) {
    let t13181 = F::new(1.0) / t13180;
    let t13182 = t225 * t13181;
    let t13183 = t3738 * t1294;
    let t13184 = t13182 * t13183;
    let t13189 = F::new(0.39512695097613069591e1) * t3732 * t3739 - F::new(0.19756347548806534796e1) * t12666 * t1215 - F::new(0.19756347548806534796e1) * t3556 * t3585 + F::new(0.39512695097613069591e1) * t3561 * t3739 - F::new(0.19756347548806534796e1) * t12673 * t1295 - F::new(0.19756347548806534796e1) * t3561 * t3791 + F::new(0.65854491829355115987e0) * t12690 * t495 + F::new(0.19756347548806534796e1) * t3552 * t1271 + F::new(0.39512695097613069591e1) * t1274 * t12696 - F::new(0.65854491829355115987e0) * t1274 * t13166 + F::new(0.65854491829355115987e0) * t460 * t13170 - F::new(0.39512695097613069591e1) * t1210 * t13174 - F::new(0.39512695097613069591e1) * t13177 * t1215 - F::new(0.39512695097613069591e1) * t1274 * t13184 + F::new(0.19756347548806534796e1) * t1204 * t3729;
    (t13181, t13183, t13184, t13189)
}
