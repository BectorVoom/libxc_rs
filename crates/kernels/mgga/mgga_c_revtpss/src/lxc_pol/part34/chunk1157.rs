//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1157/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1157<F: Float>(t107212: F, t107636: F, t1089: F, t1651: F, t1668: F, t1695: F, t19482: F, t25464: F, t25591: F, t25605: F, t25629: F, t27568: F, t27609: F, t27640: F, t27642: F, t27661: F, t27699: F, t29731: F, t29732: F, t29744: F, t29747: F, t29751: F, t29875: F, t29876: F, t6245: F, t6259: F, t6299: F, t6351: F, t7145: F, t7151: F, t7159: F, t7817: F, t7828: F, t7833: F, t99721: F, t99915: F) -> (F,) {
    let t113912 = 0.26020884564615598386e1 * t25605 * t7828 * t6299 * t1089 - 0.26020884564615598386e1 * t27661 * t29876 - 0.26020884564615598386e1 * t25629 * t29875 * t1668 * t1089 - 0.26020884564615598386e1 * t25629 * t7817 * t6299 * t1089 + 0.52041769129231196772e1 * t99915 * t29744 + 0.13010442282307799193e1 * t27640 * t27642 * t19482 * t6299 + 0.26020884564615598386e1 * t27609 * t29732 - 0.78062653693846795158e1 * t7159 * t25464 * t29731 * t1695 + 0.10408353825846239354e2 * t25591 * t7145 * t29747 * t1651 - 0.19756347548806534796e1 * t27568 * t6259 + 0.15612530738769359031e2 * t7151 * t25464 * t29751 * t1651 - 0.13010442282307799193e1 * t107636 * t7833 + 0.39512695097613069591e1 * t99721 * t6245 + 0.39512695097613069591e1 * t27699 * t6351 - 0.26020884564615598386e1 * t107212 * t7833;
    (t113912,)
}
