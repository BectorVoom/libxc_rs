//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 706/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk706<F: Float>(t4375: F, t8335: F, t1586: F, t4423: F, t5668: F, t7738: F, t7742: F, t7746: F, t2292: F, t1537: F, t4443: F, t4450: F, t5736: F, t7758: F, t7765: F, t7771: F, t7773: F, t7777: F, t7780: F, t7783: F) -> (F, F, F, F, F, F) {
    let t8336 = t4375 * t8335;
    let t8337 = t1586 * t8336;
    let t8344 = t4423 + 0.11415555555555555555e-1 * t5668 - 0.11415555555555555555e-1 * t7738 + 0.34246666666666666666e-1 * t7742 - 0.17123333333333333333e-1 * t7746;
    let t8349 = t2292 * t2292;
    let t8350 = t8349 * t1537;
    let t8365 = -0.17648625e1 * t7758 + 0.3529725e1 * t7765 + t4443 + 0.34431666666666666666e0 * t5668 - 0.34431666666666666667e0 * t7738 + 0.103295e1 * t7742 - 0.516475e0 * t7746 + 0.31558125e0 * t7771 + 0.6311625e0 * t7773 + t4450 + 0.13892666666666666667e0 * t5736 - 0.34731666666666666667e-1 * t7777 + 0.20839e0 * t7780 - 0.104195e0 * t7783;
    (t8336, t8337, t8344, t8349, t8350, t8365)
}
