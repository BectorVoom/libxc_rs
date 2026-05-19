//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 983/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk983<F: Float>(t305: F, t38812: F, t118: F, t27048: F, t326: F, t38809: F, t40602: F, t40785: F, t40788: F, t40791: F, t40804: F, t40807: F, t40809: F, t40811: F, t5266: F, t793: F, t866: F, t876: F, t8946: F, t8975: F) -> F {
    let t40814 = t305 * t38812;
    let t40816 = F::cast_from(0.11974241701863808564e0_f64) * t793 * t40602 + F::cast_from(0.59871208509319042821e-1_f64) * t305 * t40785 + F::cast_from(0.11974241701863808564e0_f64) * t305 * t40788 - F::cast_from(0.79828278012425390428e-1_f64) * t118 * t40791 + F::cast_from(0.35922725105591425692e0_f64) * t27048 * t8975 * t876 + F::cast_from(0.11974241701863808564e0_f64) * t5266 * t8946 * t866 - F::cast_from(0.59871208509319042821e-1_f64) * t326 * t38809 - t40804 - t40807 - F::cast_from(0.8980681276397856423e-1_f64) * t40809 + F::cast_from(0.59871208509319042821e-1_f64) * t305 * t40811 - F::cast_from(0.14967802127329760705e-1_f64) * t40814;
    t40816
}
