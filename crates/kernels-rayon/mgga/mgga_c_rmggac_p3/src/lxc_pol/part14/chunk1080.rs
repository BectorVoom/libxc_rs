//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 1080/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk1080(t8825: f64, t10035: f64, t34649: f64, t38310: f64, t38360: f64, t38406: f64, t38479: f64, t38533: f64, t38590: f64, t38641: f64, t38693: f64, t38735: f64, t38786: f64, t38828: f64, t38883: f64, t38940: f64, t38988: f64, t39027: f64, t39075: f64, t39149: f64, t39230: f64, t39287: f64, t39335: f64, t39398: f64, t39442: f64, t39488: f64, t39533: f64, t39579: f64, t39632: f64, t39659: f64, t39713: f64, t39766: f64, t39825: f64, t39884: f64, t39925: f64, t39987: f64, t40049: f64, t40100: f64, t40133: f64, t40203: f64, t40249: f64, t40304: f64, t40353: f64, t40405: f64, t40463: f64, t40497: f64, t40550: f64, t40592: f64, t40629: f64, t40674: f64, t40714: f64, t40766: f64, t41602: f64, t41645: f64, t41683: f64, t41732: f64, t41788: f64, t41840: f64, t41881: f64, t41924: f64, t41951: f64, t41995: f64, t42046: f64, t42103: f64, t42138: f64, t42186: f64, t42227: f64, t42274: f64, t7758: f64, t7762: f64, t8: f64, t8832: f64, t8837: f64, t9440: f64, t9492: f64) -> f64 {
    let t42282 = 0.11974241701863808564e0_f64 * t8825;
    let t42287 = -t38310 + t8 * (t40766 + t40714 + t40674 + t40629 + t40592 + t40550 + t40497 + t40463 + t40405 + t40353 + t40304 + t40249 + t40203 + t40133 + t40100 + t40049 + t39987 + t39925 + t39884 + t39825 + t39766 + t39713 + t39659 + t39632 + t39579 + t39533 + t39488 + t39442 + t39398 + t39335 + t39287 + t39230 + t39149 + t39075 + t39027 + t38988 + t38940 + t38883 + t38828 + t38786 + t38735 + t38693 + t38641 + t38590 + t38533 + t38479 + t38406 + t38360 + t42186 + t42227 + t41645 + t41732 + t42274 + t42103 + t41788 + t41840 + t41951 + t41924 + t41995 + t42138 + t41602 + t42046 + t41881 + t41683) + t10035 + t9440 + t42282 + 0.14408463291498358381e-2_f64 * t7758 - 0.20496175532535769484e-3_f64 * t7762 + t34649 - 0.31923449919973379548e-4_f64 * t8832 + 0.31923449919973379548e-4_f64 * t8837 + t9492;
    t42287
}
