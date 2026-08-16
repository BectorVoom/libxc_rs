//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 767/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk767<F: Float>(t225: F, t9725: F, t9877: F, t9908: F, t9935: F, t1891: F, t68: F, t9458: F, t776: F, t845: F, t2553: F, t824: F, t9516: F) -> (F, F, F, F) {
    let t9938 = (t9725 + t9877 + t9908 + t9935) * t225;
    let t9946 = t68 * t1891;
    let t9947 = t9946 * t9458;
    let t9950 = t845 * t776;
    let t9951 = t9950 * t2553;
    let t9954 = t824 * t9516;
    (t9938, t9947, t9951, t9954)
}
