//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1282/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1282<F: Float>(t1349: F, t26560: F, t376: F, t23478: F, t3565: F, t13135: F, t5935: F, t49562: F, t5956: F, t23405: F, t26569: F, t6584: F, t94983: F, t458: F, t6579: F, t5775: F) -> (F, F, F, F, F, F, F) {
    let t104519 = t1349 * t376 * t26560 / 9.0;
    let t104525 = t23478 * t3565;
    let t104527 = t5935 * t13135;
    let t104529 = t49562 * t5956;
    let t104532 = t23405 * t26569 / 27.0;
    let t104541 = t94983 * t6584;
    let t104547 = t6579 * t458;
    let t104549 = t104547 * t5775 / 27.0;
    (t104519, t104525, t104527, t104529, t104532, t104541, t104549)
}
