//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1392/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1392<F: Float>(t1058: F, t106182: F, t106189: F, t106197: F, t106200: F, t106204: F, t106262: F, t106311: F, t106356: F, t106392: F, t106451: F, t106499: F, t106550: F, t106595: F, t106647: F, t106703: F, t106748: F, t106795: F, t106838: F, t106872: F, t106915: F, t106963: F, t107020: F, t107065: F, t107105: F, t107152: F, t107184: F, t107231: F, t107277: F, t107325: F, t107377: F, t107425: F, t107475: F, t107521: F, t107562: F, t107604: F, t107645: F, t107692: F, t1349: F, t149: F, t2075: F, t26523: F, t26769: F, t27411: F, t28: F, t3313: F, t3414: F, t3588: F, t40591: F, t558: F, t5778: F, t5973: F, t609: F, t614: F, t9439: F) -> (F,) {
    let t107699 = -24.0 * t40591 * t27411 - 24.0 * t9439 * t26523 * t609 - 2.0 * t3414 * t5973 + 4.0 * t106182 - t1349 * t28 * t5778 * t1058 * t2075 / 3.0 - 2.0 * t106189 - 2.0 / 3.0 * t1349 * t28 * t5778 * t3588 * t558 + 8.0 * t106197 - 4.0 / 27.0 * t106200 - 2.0 * t3313 * t5973 - 2.0 * t106204 + t1349 * t28 * t26769 * t614 / 3.0 - t149 * (t106451 + t107277 + t106262 + t107425 + t107152 + t107184 + t107645 + t106915 + t106647 + t106703 + t107562 + t106499 + t107065 + t106795 + t106872 + t107604 + t106392 + t106356 + t107475 + t107521 + t107377 + t107325 + t107105 + t106311 + t106838 + t106595 + t107231 + t107020 + t106963 + t107692 + t106550 + t106748);
    (t107699,)
}
