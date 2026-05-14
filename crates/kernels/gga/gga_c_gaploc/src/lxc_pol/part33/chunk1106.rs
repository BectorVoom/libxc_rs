//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1106/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1106<F: Float>(t10981: F, t5771: F, t1445: F, t24908: F, t813: F, t935: F, t1457: F, t2103: F, t32210: F, t10817: F, t7736: F, t1853: F, t191: F, t3039: F, t7635: F, t21491: F, t8793: F) -> (F, F, F, F, F, F) {
    let t32997 = 0.14300195980740170668e1 * t5771 * t10981;
    let t33001 = 0.46011511144704899612e1 * t813 * t1445 * t24908 * t935;
    let t33004 = 0.71500979903700853338e0 * t2103 * t1457 * t32210;
    let t33009 = 0.25025342966295298669e1 * t10817 * t7736;
    let t33013 = 0.71500979903700853338e0 * t7635 * t3039 * t191 * t1853;
    let t33018 = 0.50050685932590597338e1 * t8793 * t1457 * t21491;
    (t32997, t33001, t33004, t33009, t33013, t33018)
}
