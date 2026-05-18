//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 747/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk747<F: Float>(t1391: F, t7042: F, t1305: F, t161: F, t165: F, t912: F, t2476: F, t1392: F, t2334: F, t1549: F, t1562: F, t1646: F, t2407: F, t2413: F, t2487: F, t4527: F, t4679: F, t536: F, t587: F, t6987: F, t6989: F, t6994: F, t6997: F, t7002: F, t7007: F, t7012: F, t7015: F, t7019: F, t7023: F, t7027: F, t7031: F, t7037: F, t7040: F) -> (F, F, F) {
    let t7043 = t1391 * t7042;
    let t7047 = t161 * t165 * t1305;
    let t7048 = t912 * t7047;
    let t7049 = t2476 * t7048;
    let t7051 = t1392 * t2334;
    let t7052 = t1391 * t7051;
    let t7055 = -F::new(0.51123901271894332903e0) * t6987 - F::new(0.62115540045351614476e2) * t1562 * t6989 + F::new(0.27606906686822939767e2) * t4527 * t6994 - F::new(0.71500979903700853338e0) * t6997 * t1646 + F::new(0.71500979903700853338e0) * t1549 * t2407 - F::new(0.71500979903700853338e0) * t7002 * t1646 + F::new(0.71500979903700853338e0) * t536 * t7007 + F::new(0.71500979903700853338e0) * t4679 * t2413 - F::new(0.38342925953920749676e0) * t7012 + F::new(0.38342925953920749676e0) * t7015 + F::new(0.8520650211982388817e-1) * t7019 - F::new(0.8520650211982388817e-1) * t7023 + F::new(0.29792074959875355558e-1) * t7027 - F::new(0.29792074959875355558e-1) * t7031 - F::new(0.19171462976960374838e0) * t7037 + F::new(0.19171462976960374838e0) * t7040 + F::new(0.11360866949309851756e0) * t2487 * t7043 + F::new(0.95857314884801874192e-1) * t7049 - F::new(0.11360866949309851756e0) * t587 * t7052;
    (t7047, t7049, t7055)
}
