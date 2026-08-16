//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 749/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk749(t1391: f64, t7042: f64, t1305: f64, t161: f64, t165: f64, t912: f64, t2476: f64, t1392: f64, t2334: f64, t1549: f64, t1562: f64, t1646: f64, t2407: f64, t2413: f64, t2487: f64, t4527: f64, t4679: f64, t536: f64, t587: f64, t6987: f64, t6989: f64, t6994: f64, t6997: f64, t7002: f64, t7007: f64, t7012: f64, t7015: f64, t7019: f64, t7023: f64, t7027: f64, t7031: f64, t7037: f64, t7040: f64) -> (f64, f64, f64) {
    let t7043 = t1391 * t7042;
    let t7047 = t161 * t165 * t1305;
    let t7048 = t912 * t7047;
    let t7049 = t2476 * t7048;
    let t7051 = t1392 * t2334;
    let t7052 = t1391 * t7051;
    let t7055 = -0.51123901271894332903e0_f64 * t6987 - 0.62115540045351614476e2_f64 * t1562 * t6989 + 0.27606906686822939767e2_f64 * t4527 * t6994 - 0.71500979903700853338e0_f64 * t6997 * t1646 + 0.71500979903700853338e0_f64 * t1549 * t2407 - 0.71500979903700853338e0_f64 * t7002 * t1646 + 0.71500979903700853338e0_f64 * t536 * t7007 + 0.71500979903700853338e0_f64 * t4679 * t2413 - 0.38342925953920749676e0_f64 * t7012 + 0.38342925953920749676e0_f64 * t7015 + 0.8520650211982388817e-1_f64 * t7019 - 0.8520650211982388817e-1_f64 * t7023 + 0.29792074959875355558e-1_f64 * t7027 - 0.29792074959875355558e-1_f64 * t7031 - 0.19171462976960374838e0_f64 * t7037 + 0.19171462976960374838e0_f64 * t7040 + 0.11360866949309851756e0_f64 * t2487 * t7043 + 0.95857314884801874192e-1_f64 * t7049 - 0.11360866949309851756e0_f64 * t587 * t7052;
    (t7047, t7049, t7055)
}
