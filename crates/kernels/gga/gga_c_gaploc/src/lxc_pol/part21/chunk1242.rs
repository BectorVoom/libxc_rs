//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1242/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1242<F: Float>(t1457: F, t2103: F, t32210: F, t10817: F, t7736: F, t1853: F, t191: F, t3039: F, t7635: F, t21491: F, t8793: F, t10915: F, t22242: F, t32514: F) -> (F, F, F, F, F) {
    let t33004 = F::new(0.71500979903700853338e0) * t2103 * t1457 * t32210;
    let t33009 = F::new(0.25025342966295298669e1) * t10817 * t7736;
    let t33013 = F::new(0.71500979903700853338e0) * t7635 * t3039 * t191 * t1853;
    let t33018 = F::new(0.50050685932590597338e1) * t8793 * t1457 * t21491;
    let t33021 = F::new(0.42900587942220512002e1) * t22242 * t10915 * t32514;
    (t33004, t33009, t33013, t33018, t33021)
}
