//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 998/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk998<F: Float>(t162: F, t8087: F, t1985: F, t3565: F, t10704: F, t10706: F, t10709: F, t10712: F, t10716: F, t10719: F, t10721: F, t10724: F, t10727: F, t7979: F, t7988: F, t7992: F, t8225: F, t8231: F, t8234: F) -> (F, F) {
    let t10728 = t8087 * t162;
    let t10729 = t3565 * t1985;
    let t10731 = F::cast_from(24.0_f64) * t10728 * t10729;
    let t10732 = t8225 + t10704 - t8231 - t8234 + t7979 + t10706 + t10709 + t10712 + t10716 - t10719 + t10721 + t10724 + t10727 + t10731 + t7988 + t7992;
    (t10731, t10732)
}
