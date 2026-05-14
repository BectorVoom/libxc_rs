//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 346/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk346<F: Float>(t1811: F, t225: F, t494: F, t1280: F, t1774: F, t1287: F, t1794: F, t487: F, t489: F, t1234: F, t1285: F, t1770: F, t460: F, t490: F, t1277: F, t1210: F, t1274: F, t1775: F, t495: F) -> (F, F, F, F, F, F, F) {
    let t1812 = t1811 * t225;
    let t1813 = t1812 * t494;
    let t1818 = t1280 * t1774;
    let t1822 = t487 * t1794 * t1287;
    let t1825 = t489 * t1811;
    let t1828 = 0.65854491829355115987e0 * t1770 * t490 - 0.65854491829355115987e0 * t1234 * t1818 + 0.65854491829355115987e0 * t1285 * t1822 + 0.65854491829355115987e0 * t460 * t1825;
    let t1829 = t1277 * t1828;
    let t1832 = 0.65854491829355115987e0 * t1770 * t495 - 0.65854491829355115987e0 * t1210 * t1775 + 0.65854491829355115987e0 * t460 * t1813 - 0.65854491829355115987e0 * t1274 * t1829;
    (t1813, t1818, t1822, t1825, t1828, t1829, t1832)
}
