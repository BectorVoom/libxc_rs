//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 1038/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk1038(t1986: f64, t326: f64, t559: f64, t615: f64, t7717: f64, t1737: f64, t1970: f64, t1971: f64, t209: f64, t476: f64, t880: f64, t16503: f64, t2281: f64, t34962: f64, t8425: f64) -> (f64, f64, f64) {
    let t47047 = t1986 * t326 * t559 * t615;
    let t47048 = t7717 * t47047;
    let t47054 = t1970 * t1971 * t880 * t1737 * t476 * t209;
    let t47062 = t16503 * t34962 * t2281 * t8425;
    (t47048, t47054, t47062)
}
