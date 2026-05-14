//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1136/1270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1136<F: Float>(t2617: F, t7810: F, t8802: F, t3005: F, t7344: F, t32435: F, t5241: F, t5640: F, t590: F, t10981: F, t5771: F, t1445: F, t24908: F, t813: F, t935: F, t1457: F, t2103: F, t32210: F) -> (F, F, F, F, F, F) {
    let t32983 = t7810 * t8802 * t2617;
    let t32984 = 0.38342925953920749676e0 * t32983;
    let t32986 = t7810 * t3005 * t7344;
    let t32987 = 0.19171462976960374838e0 * t32986;
    let t32991 = 0.30674340763136599742e1 * t5640 * t5241 * t32435 * t590;
    let t32997 = 0.14300195980740170668e1 * t5771 * t10981;
    let t33001 = 0.46011511144704899612e1 * t813 * t1445 * t24908 * t935;
    let t33004 = 0.71500979903700853338e0 * t2103 * t1457 * t32210;
    (t32984, t32987, t32991, t32997, t33001, t33004)
}
