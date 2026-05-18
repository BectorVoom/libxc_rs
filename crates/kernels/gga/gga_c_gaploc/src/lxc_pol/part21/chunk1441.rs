//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1441/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1441<F: Float>(t38974: F, t550: F, t2033: F, t28827: F, t28828: F, t28833: F, t28836: F, t28839: F, t28841: F, t28851: F, t28854: F, t28859: F, t28861: F, t28864: F, t28865: F, t28873: F, t33732: F, t39044: F, t549: F, t7584: F, t7585: F) -> (F, F) {
    let t39272 = t550 * t38974;
    let t39281 = -t28827 + F::new(0.38342925953920749677e0) * t28828 + F::new(0.38342925953920749677e0) * t28833 - t28836 + t28839 + t28841 - t28851 - t28854 + t33732 + t28859 + F::new(0.79445533226334281486e-1) * t2033 * t549 * t39272 - F::new(0.38342925953920749677e0) * t28861 + t28864 - F::new(0.76685851907841499354e0) * t28865 + t28873 - F::new(0.23005755572352449806e2) * t7584 * t7585 * t39044;
    (t39272, t39281)
}
