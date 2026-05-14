//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1275/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1275<F: Float>(t1284: F, t2721: F, t20237: F, t32087: F, t33416: F, t32013: F, t3936: F, t33544: F, t3748: F, t12841: F, t33547: F, t113708: F, t9446: F, t13900: F, t9804: F, t21499: F, t33383: F) -> (F, F, F, F, F, F, F, F, F) {
    let t113888 = t2721 * t1284;
    let t113902 = 0.30864197530864197531e-2 * t32087 * t20237 * t33416;
    let t113914 = t3936 * t32013;
    let t113920 = t3748 * t33544;
    let t113922 = t12841 * t33547;
    let t113923 = 0.3684876543209876543e-2 * t113922;
    let t113933 = 0.69444444444444444446e-2 * t9446 * t113708;
    let t113939 = t9446 * t13900 * t9804;
    let t113941 = t33383 * t21499;
    (t113888, t113902, t113914, t113920, t113922, t113923, t113933, t113939, t113941)
}
