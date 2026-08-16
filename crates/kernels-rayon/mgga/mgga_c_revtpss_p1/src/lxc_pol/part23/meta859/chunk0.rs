//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2749/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2749(t1812: f64, t3566: f64, t1209: f64, t13181: f64, t1774: f64, t17306: f64, t1811: f64, t21342: f64, t21333: f64, t487: f64, t488: f64, t1269: f64, t6564: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t72805 = t3566 * t1812;
    let t72808 = t1209 * t1812;
    let t72843 = t13181 * t1774;
    let t72874 = t17306 * t1811;
    let t72877 = t1209 * t21342;
    let t72894 = t21333 * t487;
    let t72927 = t17306 * t488;
    let t72933 = t6564 * t1269;
    (t72805, t72808, t72843, t72874, t72877, t72894, t72927, t72933)
}
