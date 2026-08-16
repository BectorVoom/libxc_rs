//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta420 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1594;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1595;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1596;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta420<F: Float>(t1995: F, t9223: F, t213: F, t1999: F, t1372: F, t552: F, t1307: F, t6637: F, t6888: F, t3719: F, t6968: F, t117: F, t547: F, t67: F, t6559: F, t225: F, t794: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t22865, t22867, t22881, t22882, t22883, t22884, t22886, t22887, t22888, t22891) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1594::<F>(t1995, t9223, t213, t1999, t1372, t552, t1307, t6637, t6888, t3719, t6968, t117, t547, t67);
        let t22892 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1595::<F>(t22891, t6559);
        let t22893 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1596::<F>(t225, t794);
    (t22865, t22867, t22881, t22882, t22883, t22884, t22886, t22887, t22888, t22891, t22892, t22893)
}
