//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 899/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk899<F: Float>(t21556: F, t535: F, t1311: F, t539: F, t1568: F, t6497: F, t13776: F, t41: F, t13900: F, t2321: F, t1580: F, t4381: F, t6473: F, t6449: F, t1308: F, t3969: F, t6458: F, sigma0: F) -> (F, F, F, F, F, F, F, F) {
    let t21558 = 0.17990788716177317213e-1 * t535 * t21556;
    let t21559 = t1311 * t539;
    let t21567 = 0.17990788716177317213e-1 * t1568 * t6497;
    let t21592 = t41 * t13776;
    let t21620 = t13900 * t2321;
    let t21621 = t1580 * t21620;
    let t21626 = 0.15991812192157615301e-1 * t4381 * t6473;
    let t21661 = t6449 * sigma0;
    let t21662 = t21661 * t1308;
    let t21665 = t6458 * t3969;
    (t21558, t21559, t21567, t21592, t21621, t21626, t21662, t21665)
}
