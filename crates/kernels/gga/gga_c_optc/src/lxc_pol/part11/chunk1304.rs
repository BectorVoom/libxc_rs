//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1304/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1304<F: Float>(t13890: F, t4818: F, t7681: F, t3780: F, t49939: F, t845: F, t16943: F, t3788: F, t1375: F, t49995: F, t23801: F, t23804: F, t56677: F) -> (F, F, F, F, F) {
    let t57233 = F::cast_from(0.57894567559743977359e3_f64) * t7681 * t13890 * t4818;
    let t57236 = F::cast_from(0.69263023597503453196e2_f64) * t845 * t49939 * t3780;
    let t57238 = F::cast_from(0.41015588084031179722e4_f64) * t3788 * t16943;
    let t57240 = F::cast_from(0.23392893589820816284e1_f64) * t49995 * t1375;
    let t57244 = F::cast_from(0.91080982599109921211e5_f64) * t845 * t23801 * t56677 * t23804;
    (t57233, t57236, t57238, t57240, t57244)
}
