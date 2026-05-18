//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 827/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk827<F: Float>(t3521: F, t7846: F, t425: F, t7757: F, t7870: F, t7862: F, t2059: F, t2083: F, t7850: F, t7854: F, t1417: F, t7879: F) -> (F, F, F, F, F, F, F, F) {
    let t26579 = t3521 * t7846;
    let t26590 = t425 * t7757;
    let t26600 = t3521 * t7870;
    let t26602 = t3521 * t7862;
    let t26617 = t2059 * t2083;
    let t26632 = t3521 * t7850;
    let t26692 = t3521 * t7854;
    let t26710 = t1417 * t7879;
    (t26579, t26590, t26600, t26602, t26617, t26632, t26692, t26710)
}
