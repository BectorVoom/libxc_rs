//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1277/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1277<F: Float>(t2126: F, t4292: F, t1493: F, t68: F, t640: F, t119457: F, t122886: F, t122890: F, t122893: F, t122901: F, t124193: F, t124200: F, t124210: F, t124220: F, t124238: F, t125260: F, t125268: F, t125279: F, t125336: F, t129180: F, t129232: F, t129236: F, t1469: F, t32795: F, t32802: F, t32806: F, t33265: F, t33268: F, t33270: F, t33275: F, t33277: F, t34402: F, t34765: F, t34771: F, t4186: F, t4237: F, t644: F, t8442: F, t8621: F, t8737: F, t8881: F, t95334: F) -> (F, F) {
    let t129470 = t2126 * t4292;
    let t130808 = t68 * t1493;
    let t130831 = t640 * t68;
    let t130845 = F::new(5.0) / F::new(18.0) * t122901 * t34765 + F::new(5.0) / F::new(18.0) * t122890 * t34765 + F::new(5.0) / F::new(18.0) * t32802 * t8442 * t95334 * t1469 + F::new(5.0) / F::new(18.0) * t32802 * t8442 * t33268 * t4186 - F::new(5.0) / F::new(9.0) * t129232 * t8621 * t8881 * t1469 + F::new(5.0) / F::new(18.0) * t129236 * t33270 + F::new(5.0) / F::new(6.0) * t122886 * t119457 * t130808 * t644 - F::new(10.0) / F::new(9.0) * t124210 + F::new(10.0) / F::new(27.0) * t124220 + F::new(5.0) / F::new(9.0) * t32802 * t124200 * t125279 - F::new(5.0) / F::new(3.0) * t122893 * t124193 * t125336 - F::new(5.0) / F::new(3.0) * t122893 * t124193 * t125260 + F::new(5.0) / F::new(9.0) * t32802 * t124200 * t125268 - F::new(5.0) / F::new(36.0) * t32795 * t34771 - F::new(5.0) / F::new(36.0) * t32806 * t34771 - F::new(5.0) / F::new(36.0) * t8737 * t8621 * t130831 * t1493 - F::new(5.0) / F::new(36.0) * t8737 * t8621 * t33275 * t4237 + F::new(5.0) / F::new(12.0) * t129180 * t33265 - F::new(5.0) / F::new(36.0) * t34402 * t33277 - F::new(20.0) / F::new(27.0) * t124238;
    (t129470, t130845)
}
