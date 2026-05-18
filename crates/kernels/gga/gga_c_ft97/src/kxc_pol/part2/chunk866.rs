//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 866/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk866<F: Float>(t1113: F, t2426: F, t1127: F, t9681: F, t13452: F, t2379: F, t13448: F, t13407: F, t3785: F, t122: F, t13402: F, t3751: F, t709: F) -> (F, F, F, F, F, F, F) {
    let t13491 = t2426 * t1113;
    let t13495 = t9681 * t1127;
    let t13499 = t2379 * t13452;
    let t13502 = t2379 * t13448;
    let t13505 = t3785 * t13407;
    let t13508 = t1127 * t122;
    let t13509 = t13508 * t13402;
    let t13515 = t3751 * t709;
    (t13491, t13495, t13499, t13502, t13505, t13509, t13515)
}
