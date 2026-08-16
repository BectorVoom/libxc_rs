//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 983/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk983(t305: f64, t38812: f64, t118: f64, t27048: f64, t326: f64, t38809: f64, t40602: f64, t40785: f64, t40788: f64, t40791: f64, t40804: f64, t40807: f64, t40809: f64, t40811: f64, t5266: f64, t793: f64, t866: f64, t876: f64, t8946: f64, t8975: f64) -> f64 {
    let t40814 = t305 * t38812;
    let t40816 = 0.11974241701863808564e0_f64 * t793 * t40602 + 0.59871208509319042821e-1_f64 * t305 * t40785 + 0.11974241701863808564e0_f64 * t305 * t40788 - 0.79828278012425390428e-1_f64 * t118 * t40791 + 0.35922725105591425692e0_f64 * t27048 * t8975 * t876 + 0.11974241701863808564e0_f64 * t5266 * t8946 * t866 - 0.59871208509319042821e-1_f64 * t326 * t38809 - t40804 - t40807 - 0.8980681276397856423e-1_f64 * t40809 + 0.59871208509319042821e-1_f64 * t305 * t40811 - 0.14967802127329760705e-1_f64 * t40814;
    t40816
}
