//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1297/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1297<F: Float>(t115710: F, t3936: F, t14609: F, t21499: F, t533: F, t33766: F, t9528: F, t114715: F, t115725: F, t14962: F, t1597: F, t33807: F, t9535: F, t115075: F, t9536: F, t114774: F) -> (F, F, F, F, F, F, F, F, F) {
    let t115849 = t3936 * t115710;
    let t115858 = t14609 * t533 * t21499;
    let t115871 = t33766 * t9528;
    let t115883 = 0.23214722222222222222e-2 * t114715;
    let t115890 = t3936 * t115725;
    let t115913 = t14962 * t1597;
    let t115926 = t33807 * t9535;
    let t115932 = 0.34722222222222222222e-2 * t9536 * t115075;
    let t115941 = 0.61905925925925925925e-2 * t114774;
    (t115849, t115858, t115871, t115883, t115890, t115913, t115926, t115932, t115941)
}
