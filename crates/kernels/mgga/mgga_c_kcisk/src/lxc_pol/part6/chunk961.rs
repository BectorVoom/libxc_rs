//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 961/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk961<F: Float>(t23976: F, t23978: F, t24608: F, t2648: F, t28797: F, t28803: F, t28807: F, t28811: F, t28815: F, t28818: F, t28953: F, t29759: F, t29981: F, t5445: F) -> F {
    let t30020 = F::new(0.18571777777777777778e-1) * t28797 + F::new(0.18571777777777777778e-1) * t23976 - F::new(0.11607361111111111111e-2) * t28803 - F::new(0.92858888888888888888e-2) * t28807 - F::new(0.15476481481481481482e-1) * t28811 - F::new(0.11607361111111111111e-1) * t28815 - F::new(0.69644166666666666666e-2) * t28818 + F::new(0.46429444444444444443e-2) * t23978 - F::new(0.579e0) * t24608 * t2648 - F::new(0.223494e0) * t5445 * t29759 - F::new(0.17411041666666666666e-2) * t28953 + F::new(0.223494e0) * t5445 * t29981;
    t30020
}
