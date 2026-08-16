//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1196/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1196(t84508: f64, t84529: f64, t84551: f64, t84572: f64, t3787: f64, t7191: f64, t1338: f64, t24063: f64, t1336: f64, t1352: f64, t24116: f64, t3793: f64, t3851: f64, t544: f64, t553: f64, t81055: f64, t81059: f64, t81061: f64, t81066: f64, t81069: f64, t81076: f64, t81080: f64, t81083: f64, t81087: f64, t81092: f64, t81097: f64, t81099: f64, t84480: f64, t84481: f64) -> (f64, f64) {
    let t84574 = t84508 + t84529 + t84551 + t84572;
    let t84577 = t3787 * t7191;
    let t84581 = t1338 * t24063;
    let t84585 = 0.29608813203268075857e0_f64 * t81055 - 0.16449340668482264365e-1_f64 * t81059 - 0.38381794893125283518e0_f64 * t81061 - 3.0_f64 * t1336 * t24116 * t3851 + 0.49348022005446793095e-1_f64 * t81066 - 0.24674011002723396548e-1_f64 * t81069 - t84480 - t84481 + 0.15626873635058151147e0_f64 * t81076 - 0.31253747270116302294e0_f64 * t81080 + 0.9869604401089358619e-1_f64 * t81083 - 0.39478417604357434476e0_f64 * t81087 - 0.49348022005446793095e-1_f64 * t81092 - 0.49348022005446793095e-1_f64 * t81097 + 0.11514538467937585055e0_f64 * t81099 + t544 * t553 * t84574 + 6.0_f64 * t1336 * t84577 * t3793 - 3.0_f64 * t1336 * t84581 * t1352;
    (t84574, t84585)
}
