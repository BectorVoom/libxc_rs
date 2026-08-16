//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 492/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk492<F: Float>(t1811: F, t225: F, t494: F, t1280: F, t1774: F, t1287: F, t1794: F, t487: F, t489: F, t1234: F, t1285: F, t1770: F, t460: F, t490: F) -> (F, F, F, F, F) {
    let t1812 = t1811 * t225;
    let t1813 = t1812 * t494;
    let t1818 = t1280 * t1774;
    let t1822 = t487 * t1794 * t1287;
    let t1825 = t489 * t1811;
    let t1828 = F::cast_from(0.65854491829355115987e0_f64) * t1770 * t490 - F::cast_from(0.65854491829355115987e0_f64) * t1234 * t1818 + F::cast_from(0.65854491829355115987e0_f64) * t1285 * t1822 + F::cast_from(0.65854491829355115987e0_f64) * t460 * t1825;
    (t1813, t1818, t1822, t1825, t1828)
}
