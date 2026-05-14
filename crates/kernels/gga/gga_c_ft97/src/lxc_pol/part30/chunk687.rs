//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 687/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk687<F: Float>(t33458: F, t33474: F, t33344: F, t33349: F, t33455: F, t33463: F, t33467: F, t33471: F, t33479: F, t33483: F, t33487: F, t33518: F, t33517: F, t258: F, t1403: F, t33245: F, t33248: F, t33255: F, t33259: F, t33264: F, t33269: F, t33272: F, t33275: F, t33279: F, t33490: F, t33496: F, t33499: F, t33504: F, t5996: F, t6002: F, t6005: F, t6011: F, t6064: F, t6068: F, t7437: F, t7491: F) -> (F, F, F, F, F) {
    let t33522 = 2.0 / 9.0 * t33458;
    let t33526 = t33474 / 9.0;
    let t33530 = t33518 + t33344 / 18.0 + t33349 / 3.0 - t33455 / 6.0 - t33522 - 2.0 / 9.0 * t33463 - 2.0 * t33467 + 4.0 / 3.0 * t33471 + t33526 + t33479 / 9.0 + 2.0 / 3.0 * t33483 - t33487 / 3.0;
    let t33531 = t33517 + t33530;
    let t33532 = t33531 * t258;
    let t33534 = t1403 * t33245 - 2.0 / 3.0 * t1403 * t33248 - t7437 * t6011 / 3.0 - t1403 * t33255 / 3.0 + t1403 * t33259 / 3.0 + t7437 * t6068 / 6.0 - 4.0 * t33264 + t5996 * t7491 / 3.0 + t1403 * t33269 / 3.0 - 4.0 * t33272 - 2.0 * t33275 - 2.0 / 3.0 * t1403 * t33279 - 2.0 * t33490 + t7437 * t6064 / 6.0 + t6002 * t33496 / 9.0 - t33499 * t6005 / 18.0 - t6002 * t33504 / 9.0 + 2.0 * t33532;
    (t33522, t33526, t33531, t33532, t33534)
}
