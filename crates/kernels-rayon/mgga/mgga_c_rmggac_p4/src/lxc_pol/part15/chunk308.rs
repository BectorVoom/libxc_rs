//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 308/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk308(t1845: f64, t1846: f64, t1157: f64, t1442: f64, t1835: f64, t1839: f64, t198: f64, t454: f64, t589: f64, t201: f64, t597: f64, t615: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1847 = t1845 + t1846;
    let t1856 = -0.32163648644302209643e2_f64 * t1847 * t198 + 0.19298189186581325786e3_f64 * t1442 * t589 - 0.38596378373162651572e3_f64 * t1157 * t1839 + 0.96490945932906628929e2_f64 * t454 * t1835;
    let t1857 = t1856 * t201;
    let t1859 = t597 * t597;
    let t1860 = t1859 * t201;
    let t1864 = t615 * t615;
    (t1847, t1856, t1857, t1859, t1860, t1864)
}
