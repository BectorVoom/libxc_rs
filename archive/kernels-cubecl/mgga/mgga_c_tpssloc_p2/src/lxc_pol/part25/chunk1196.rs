//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1196/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1196<F: Float>(t84508: F, t84529: F, t84551: F, t84572: F, t3787: F, t7191: F, t1338: F, t24063: F, t1336: F, t1352: F, t24116: F, t3793: F, t3851: F, t544: F, t553: F, t81055: F, t81059: F, t81061: F, t81066: F, t81069: F, t81076: F, t81080: F, t81083: F, t81087: F, t81092: F, t81097: F, t81099: F, t84480: F, t84481: F) -> (F, F) {
    let t84574 = t84508 + t84529 + t84551 + t84572;
    let t84577 = t3787 * t7191;
    let t84581 = t1338 * t24063;
    let t84585 = F::cast_from(0.29608813203268075857e0_f64) * t81055 - F::cast_from(0.16449340668482264365e-1_f64) * t81059 - F::cast_from(0.38381794893125283518e0_f64) * t81061 - F::cast_from(3.0_f64) * t1336 * t24116 * t3851 + F::cast_from(0.49348022005446793095e-1_f64) * t81066 - F::cast_from(0.24674011002723396548e-1_f64) * t81069 - t84480 - t84481 + F::cast_from(0.15626873635058151147e0_f64) * t81076 - F::cast_from(0.31253747270116302294e0_f64) * t81080 + F::cast_from(0.9869604401089358619e-1_f64) * t81083 - F::cast_from(0.39478417604357434476e0_f64) * t81087 - F::cast_from(0.49348022005446793095e-1_f64) * t81092 - F::cast_from(0.49348022005446793095e-1_f64) * t81097 + F::cast_from(0.11514538467937585055e0_f64) * t81099 + t544 * t553 * t84574 + F::cast_from(6.0_f64) * t1336 * t84577 * t3793 - F::cast_from(3.0_f64) * t1336 * t84581 * t1352;
    (t84574, t84585)
}
