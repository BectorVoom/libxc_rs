//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 869/951 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk869<F: Float>(t40802: F, t5148: F, t333: F, t8915: F, t4669: F, t2392: F, t876: F, t27048: F, t551: F, t7858: F, t305: F, t38812: F, t118: F, t326: F, t38809: F, t40602: F, t40785: F, t40788: F, t40791: F, t5266: F, t793: F, t866: F, t8946: F, t8975: F) -> (F, F, F, F) {
    let t40803 = t5148 * t40802;
    let t40804 = 0.15965655602485078085e0 * t40803;
    let t40805 = t8915 * t333;
    let t40806 = t4669 * t40805;
    let t40807 = 0.23948483403727617128e0 * t40806;
    let t40808 = t2392 * t876;
    let t40809 = t27048 * t40808;
    let t40811 = t7858 * t551;
    let t40814 = t305 * t38812;
    let t40816 = 0.11974241701863808564e0 * t793 * t40602 + 0.59871208509319042821e-1 * t305 * t40785 + 0.11974241701863808564e0 * t305 * t40788 - 0.79828278012425390428e-1 * t118 * t40791 + 0.35922725105591425692e0 * t27048 * t8975 * t876 + 0.11974241701863808564e0 * t5266 * t8946 * t866 - 0.59871208509319042821e-1 * t326 * t38809 - t40804 - t40807 - 0.8980681276397856423e-1 * t40809 + 0.59871208509319042821e-1 * t305 * t40811 - 0.14967802127329760705e-1 * t40814;
    (t40805, t40808, t40811, t40816)
}
