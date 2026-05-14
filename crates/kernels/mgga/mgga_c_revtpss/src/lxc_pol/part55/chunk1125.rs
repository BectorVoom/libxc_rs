//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1125/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1125<F: Float>(t33281: F, t34402: F, t34410: F, t1497: F, t32798: F, t33280: F, t8621: F, t124217: F, t1493: F, t8737: F, t68: F, t124235: F, t1469: F, t32802: F, t8442: F, t119457: F, t122886: F, t122911: F, t122918: F, t124246: F, t124255: F, t124256: F, t129157: F, t129160: F, t129165: F, t129169: F, t129193: F, t129213: F, t129216: F, t33265: F, t33270: F, t33277: F, t34761: F, t4241: F, t640: F, t644: F, t8881: F, t8882: F) -> (F,) {
    let t130848 = t34402 * t33281;
    let t130858 = t34410 * t33281;
    let t130862 = t32798 * t8621 * t33280 * t1497;
    let t130866 = t8737 * t8621 * t124217 * t1493;
    let t130882 = t68 * t1497;
    let t130893 = t32802 * t8442 * t124235 * t1469;
    let t130895 = 5.0 / 27.0 * t124246 - t124255 + 5.0 / 27.0 * t124256 + 5.0 / 27.0 * t130848 - 5.0 / 72.0 * t129157 * t8882 - 5.0 / 72.0 * t129160 * t8882 - 5.0 / 72.0 * t129165 * t8882 - 5.0 / 72.0 * t129169 * t8882 + 5.0 / 27.0 * t130858 - 10.0 / 9.0 * t130862 + 10.0 / 27.0 * t130866 + 5.0 / 12.0 * t122911 * t34761 + 5.0 / 12.0 * t122918 * t34761 + 5.0 / 12.0 * t32798 * t8621 * t8881 * t4241 + 5.0 / 12.0 * t129193 * t33265 - 5.0 / 36.0 * t34410 * t33277 + 5.0 / 18.0 * t129213 * t33270 - 35.0 / 12.0 * t129216 * t8442 * t130882 * t644 + 5.0 / 6.0 * t122886 * t119457 * t130882 * t640 - 20.0 / 27.0 * t130893;
    (t130895,)
}
