//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1251/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1251<F: Float>(t100186: F, t100378: F, t11442: F, t11468: F, t11472: F, t11854: F, t11863: F, t1570: F, t1643: F, t1647: F, t1651: F, t1876: F, t1901: F, t23265: F, t25595: F, t25598: F, t25919: F, t25933: F, t26305: F, t26436: F, t3188: F, t379: F, t38711: F, t39167: F, t47231: F, t47404: F, t5743: F, t60426: F, t6478: F, t6538: F, t6547: F, t8557: F, t91928: F) -> (F,) {
    let t103416 = -2.0 / 9.0 * t91928 + 4.0 / 9.0 * t1901 * t11854 * t6547 * t1647 - 2.0 / 9.0 * t1901 * t38711 * t26305 - t1901 * t8557 * t6478 * t1651 / 9.0 - 2.0 / 27.0 * t1901 * t39167 * t6478 * t1643 - 4.0 / 9.0 * t1901 * t47231 * t25933 - 4.0 / 9.0 * t1901 * t11863 * t100186 + 8.0 / 3.0 * t1901 * t60426 * t6538 * t1876 - 4.0 / 9.0 * t1901 * t11854 * t25595 * t379 - 4.0 / 9.0 * t1901 * t11854 * t25598 * t379 - 4.0 / 9.0 * t1901 * t47231 * t25919 - 2.0 / 9.0 * t1901 * t11854 * t23265 * t11442 + 2.0 / 3.0 * t1901 * t11468 * t100378 - 4.0 / 9.0 * t1901 * t47404 * t26436 - 4.0 / 9.0 * t1901 * t11472 * t5743 * t1570 * t3188;
    (t103416,)
}
