//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1074/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1074<F: Float>(t529: F, t21559: F, t442: F, t18681: F, t1568: F, t6497: F, t19710: F, t41: F, t1216: F, t260: F, t67: F, t13776: F, t19881: F, t19904: F, t6443: F, t1287: F, t2153: F, t2308: F, t382: F, t4144: F, t4148: F, t4354: F, t525: F, t6431: F, t6442: F, t6444: F) -> (F, F, F) {
    let t530 = t529 < -0.66725e-1;
    let t21560 = t21559 * t442;
    let t21561 = t18681 * t21560;
    let t21567 = 0.17990788716177317213e-1 * t1568 * t6497;
    let t21572 = t19710 * t41;
    let t21589 = t260 * t67 * t1216;
    let t21592 = t41 * t13776;
    let t21593 = t21592 * t19881;
    let t21596 = t6443 * t19904;
    let t21600 = piecewise3(t530, 0.0, 10.0 / 9.0 * t525 * t21572 * t382 - 20.0 / 27.0 * t525 * t6431 * t1287 + 40.0 / 81.0 * t525 * t2308 * t4144 - 10.0 / 27.0 * t525 * t2308 * t4148 - 10.0 / 27.0 * t525 * t4354 * t2153 + 80.0 / 81.0 * t21589 * t6444 - 280.0 / 243.0 * t6442 * t21593 + 40.0 / 81.0 * t6442 * t21596);
    (t21561, t21567, t21600)
}
