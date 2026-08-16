//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta612 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1925;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1926;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta612<F: Float>(t22863: F, t7737: F, t26448: F, t90497: F, t215: F, t6916: F, t225: F, t3787: F, t562: F, t16313: F, t22751: F, t26385: F, t16068: F, t1992: F, t6976: F, t26395: F, t3719: F, t6637: F, t6888: F, t16307: F, t90915: F, t1307: F, t26331: F, t26446: F, t90818: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t91000, t91002, t91004, t91005, t91008, t91010) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1925::<F>(t22863, t7737, t26448, t90497, t215, t6916, t225, t3787, t562, t16313, t22751, t26385);
        let (t91014, t91025, t91036, t91048) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1926::<F>(t16068, t1992, t6976, t26395, t3719, t6637, t6888, t16307, t90915, t91004, t1307, t26331, t26446, t90818);
    (t91000, t91002, t91005, t91008, t91010, t91014, t91025, t91036, t91048)
}
