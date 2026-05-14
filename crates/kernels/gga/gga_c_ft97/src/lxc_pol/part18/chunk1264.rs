//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1264/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1264<F: Float>(t1882: F, t26230: F, t26398: F, t1326: F, t8275: F, t26368: F, t8392: F, t100259: F, t110: F, t11438: F, t11490: F, t11613: F, t11983: F, t11989: F, t1871: F, t1901: F, t23167: F, t23323: F, t25990: F, t26349: F, t26373: F, t3238: F, t3291: F, t446: F, t452: F, t499: F, t5635: F, t93828: F, t93830: F, t93841: F, t93843: F) -> (F,) {
    let t103918 = 4.0 / 9.0 * t1882 * t26230;
    let t103920 = 4.0 / 9.0 * t1882 * t26398;
    let t103927 = t8275 * t1326;
    let t103936 = 4.0 / 9.0 * t8392 * t26368;
    let t103947 = -8.0 / 27.0 * t93828 + 2.0 / 9.0 * t93830 + 4.0 / 3.0 * t446 * t1871 * t499 * t25990 + 4.0 / 3.0 * t446 * t1871 * t110 * t100259 - t103918 - t103920 - 2.0 / 3.0 * t1901 * t23323 * t11438 - 2.0 / 27.0 * t1901 * t26349 * t11983 - 10.0 / 81.0 * t1901 * t103927 * t11989 + 2.0 * t1901 * t11490 * t26373 * t11613 + t103936 + t446 * t452 * t3238 * t23167 / 3.0 + t93841 / 9.0 + 4.0 / 3.0 * t446 * t1871 * t3291 * t5635 - 4.0 / 9.0 * t93843;
    (t103947,)
}
