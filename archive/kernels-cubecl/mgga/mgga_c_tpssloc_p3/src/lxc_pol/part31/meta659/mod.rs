//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta659 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1943;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1944;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta659<F: Float>(t23127: F, t5628: F, t16985: F, t6621: F, t1516: F, t87321: F, t25068: F, t4261: F, t5624: F, t23133: F, t87340: F, t16673: F, t6620: F, t849: F, t23083: F, t28375: F, t28396: F, t81835: F, t58853: F, t6605: F, t828: F, t9972: F, t4250: F, t87199: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t98818, t98820, t98822, t98824, t98826, t98828, t98830, t98832) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1943::<F>(t23127, t5628, t16985, t6621, t1516, t87321, t25068, t4261, t5624, t23133, t87340, t16673, t6620);
        let (t98833, t98836, t98838, t98842, t98844) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1944::<F>(t849, t98832, t23083, t28375, t28396, t81835, t58853, t6605, t828, t9972, t4250, t87199);
    (t98818, t98820, t98822, t98824, t98826, t98828, t98830, t98833, t98836, t98838, t98842, t98844)
}
