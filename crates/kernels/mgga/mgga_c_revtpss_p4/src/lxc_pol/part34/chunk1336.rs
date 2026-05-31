//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1336/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1336<F: Float>(t114812: F, t1937: F, t29508: F, t7735: F, t1907: F, t6816: F, t25082: F, t8717: F, t114768: F, t114770: F, t114773: F, t114775: F, t114779: F, t114783: F, t114785: F, t114787: F, t114790: F, t114794: F, t114803: F, t114807: F, t1502: F, t1518: F, t2007: F, t22633: F, t28030: F, t29986: F, t30119: F, t4248: F, t5921: F, t651: F) -> F {
    let t114814 = F::cast_from(2.0_f64) * t114812 * t1937;
    let t114816 = F::cast_from(6.0_f64) * t29508 * t7735;
    let t114820 = t6816 * t1907;
    let t114823 = F::cast_from(9.0_f64) * t25082 * t8717 * t114820;
    let t114824 = -F::cast_from(6.0_f64) * t1518 * t29986 * t651 - F::cast_from(2.0_f64) * t2007 * t22633 * t651 - F::cast_from(3.0_f64) * t1502 * t29986 - F::cast_from(6.0_f64) * t28030 * t5921 - F::cast_from(6.0_f64) * t30119 * t4248 + t114768 + t114770 - t114773 + t114775 + t114779 + t114783 - t114785 - t114787 - t114790 + t114794 - t114803 + t114807 - t114814 - t114816 - t114823;
    t114824
}
