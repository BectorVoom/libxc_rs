//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 945/951 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk945<F: Float>(t8825: F, t10035: F, t34649: F, t38310: F, t38360: F, t38406: F, t38479: F, t38533: F, t38590: F, t38641: F, t38693: F, t38735: F, t38786: F, t38828: F, t38883: F, t38940: F, t38988: F, t39027: F, t39075: F, t39149: F, t39230: F, t39287: F, t39335: F, t39398: F, t39442: F, t39488: F, t39533: F, t39579: F, t39632: F, t39659: F, t39713: F, t39766: F, t39825: F, t39884: F, t39925: F, t39987: F, t40049: F, t40100: F, t40133: F, t40203: F, t40249: F, t40304: F, t40353: F, t40405: F, t40463: F, t40497: F, t40550: F, t40592: F, t40629: F, t40674: F, t40714: F, t40766: F, t41602: F, t41645: F, t41683: F, t41732: F, t41788: F, t41840: F, t41881: F, t41924: F, t41951: F, t41995: F, t42046: F, t42103: F, t42138: F, t42186: F, t42227: F, t42274: F, t7758: F, t7762: F, t8: F, t8832: F, t8837: F, t9440: F, t9492: F) -> (F,) {
    let t42282 = 0.11974241701863808564e0 * t8825;
    let t42287 = -t38310 + t8 * (t40766 + t40714 + t40674 + t40629 + t40592 + t40550 + t40497 + t40463 + t40405 + t40353 + t40304 + t40249 + t40203 + t40133 + t40100 + t40049 + t39987 + t39925 + t39884 + t39825 + t39766 + t39713 + t39659 + t39632 + t39579 + t39533 + t39488 + t39442 + t39398 + t39335 + t39287 + t39230 + t39149 + t39075 + t39027 + t38988 + t38940 + t38883 + t38828 + t38786 + t38735 + t38693 + t38641 + t38590 + t38533 + t38479 + t38406 + t38360 + t42186 + t42227 + t41645 + t41732 + t42274 + t42103 + t41788 + t41840 + t41951 + t41924 + t41995 + t42138 + t41602 + t42046 + t41881 + t41683) + t10035 + t9440 + t42282 + 0.14408463291498358381e-2 * t7758 - 0.20496175532535769484e-3 * t7762 + t34649 - 0.31923449919973379548e-4 * t8832 + 0.31923449919973379548e-4 * t8837 + t9492;
    (t42287,)
}
