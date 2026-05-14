//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 663/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk663<F: Float>(t9571: F, t9665: F, t420: F, t701: F, t2320: F, t9592: F, t704: F, t8608: F, t9637: F, t9639: F, t9642: F, t9645: F, t9648: F, t9655: F, t9660: F, t9663: F) -> (F, F, F, F, F, F) {
    let t9666 = t9665 * t9571;
    let t9667 = t420 * t9666;
    let t9668 = t701 * t9667;
    let t9670 = t2320 * t9592;
    let t9671 = t701 * t9670;
    let t9673 = t704 * t8608;
    let t9674 = t420 * t9673;
    let t9675 = t701 * t9674;
    let t9677 = t9637 - 0.42562405586419753086e-2 * t9639 + 0.85124811172839506172e-2 * t9642 - 0.12768721675925925926e-1 * t9645 + 0.63843608379629629629e-2 * t9648 + 0.19862455940329218107e-1 * t9655 - 0.51074886703703703704e-1 * t9660 + 0.25537443351851851852e-1 * t9663 + 0.38306165027777777778e-1 * t9668 - 0.38306165027777777778e-1 * t9671 + 0.6384360837962962963e-2 * t9675;
    (t9666, t9668, t9671, t9673, t9675, t9677)
}
