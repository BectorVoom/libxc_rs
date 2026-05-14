//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 872/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk872<F: Float>(t2371: F, t35516: F, t1403: F, t35549: F, t681: F, t35752: F, t35546: F, t761: F, t766: F, t35281: F, t10052: F, t140664: F, t140684: F, t193: F, t24191: F, t27956: F, t33253: F, t35285: F, t35604: F, t35779: F, t41409: F, t6009: F, t6011: F, t6062: F, t6187: F, t6930: F, t6945: F) -> (F, F, F) {
    let t149920 = t2371 * t35516;
    let t149926 = t1403 * t681 * t35549;
    let t149929 = t1403 * t681 * t35752;
    let t149949 = t35546 * t761;
    let t149950 = t149949 * t766;
    let t149953 = t1403 * t681 * t35281;
    let t149959 = -t1403 * t193 * t149920 * t6009 / 3.0 + t149926 / 9.0 - t149929 / 18.0 - t140664 / 18.0 - t1403 * t193 * t33253 * t27956 / 3.0 + 48.0 * t41409 * t35604 * t766 - 24.0 * t10052 * t6930 * t6187 - t35779 * t6011 / 3.0 - t140684 / 9.0 + t1403 * t193 * t6062 * t6945 / 3.0 - 2.0 * t149950 - t149953 / 18.0 - 2.0 / 3.0 * t1403 * t193 * t24191 * t35285;
    (t149920, t149950, t149959)
}
