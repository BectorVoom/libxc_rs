//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1419/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1419<F: Float>(t34077: F, t538: F, t7623: F, t20582: F, t20773: F, t26106: F, t26109: F, t26116: F, t26119: F, t2667: F, t2719: F, t2892: F, t30570: F, t30572: F, t30577: F, t30579: F, t30599: F, t551: F, t552: F, t6449: F, t9367: F, t9983: F, t9987: F) -> (F,) {
    let t34429 = t7623 * t538 * t34077;
    let t34431 = -0.15602799132097683414e1 * t6449 * t551 * t552 * t2892 * t2719 + 0.69345773920434148506e0 * t30570 + 0.38415120233790484326e0 * t30572 + 0.17798748639578098116e2 * t26106 - t26109 - 0.13002332610081402845e0 * t2667 * t9367 - 0.2600466522016280569e0 * t20773 * t9983 + 0.1047928639570397803e0 * t30577 - 0.17563392970889009433e0 * t30579 + t26116 + t26119 - 0.69345773920434148506e0 * t30599 - 0.2600466522016280569e1 * t20582 * t9987 + 0.82318114786693894983e-2 * t34429;
    (t34431,)
}
