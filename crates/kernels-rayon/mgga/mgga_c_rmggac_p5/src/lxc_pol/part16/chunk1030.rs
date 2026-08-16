//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1030/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1030(t1525: f64, t1971: f64, t511: f64, t558: f64, t7230: f64, t1737: f64, t495: f64, t880: f64, t10018: f64, t7244: f64, t7255: f64, t9985: f64) -> (f64, f64, f64, f64) {
    let t47505 = t7230 * t1971 * t511 * t558 * t1525;
    let t47510 = t7230 * t1971 * t880 * t1737 * t495;
    let t47512 = t7244 * t10018;
    let t47516 = t7255 * t9985;
    (t47505, t47510, t47512, t47516)
}
