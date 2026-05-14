//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1117/1270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1117<F: Float>(t32682: F, t1944: F, t3437: F, t24745: F, t5539: F, t9647: F, t123: F, t24884: F, t2563: F, t10697: F, t7173: F, t10679: F, t10789: F, t1897: F, t29631: F, t32669: F, t32671: F, t32674: F, t32676: F, t32679: F, t32681: F, t5227: F, t5524: F, t5836: F) -> (F,) {
    let t32683 = 0.85450291446024714264e-3 * t32682;
    let t32684 = t1944 * t3437;
    let t32685 = 0.99692006687028833308e-3 * t32684;
    let t32690 = t9647 * t5539 * t24745;
    let t32691 = 0.64087718584518535698e-3 * t32690;
    let t32692 = t24884 * t123;
    let t32694 = t9647 * t32692 * t2563;
    let t32695 = 0.19226315575355560709e-2 * t32694;
    let t32697 = t9647 * t10697 * t7173;
    let t32698 = 0.96131577876777803547e-3 * t32697;
    let t32701 = -0.8545029144602471425e-3 * t5524 * t10679 - t32669 - t32671 + t32674 + t32676 + t29631 - t32679 - t32681 - t32683 + t32685 + 0.46143157380853345702e-1 * t1897 * t10789 * t5836 + t32691 - t32695 - t32698 + 0.17090058289204942853e-2 * t5227 * t10679;
    (t32701,)
}
