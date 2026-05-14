//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 966/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk966<F: Float>(t1476: F, t28719: F, t1486: F, t193: F, t2781: F, t33978: F, t4255: F, t10248: F, t446: F, t4129: F, t7584: F, t10570: F, t6260: F, t7021: F, t4226: F, t6308: F, t7611: F, t852: F) -> (F, F, F, F, F, F, F, F, F) {
    let t152767 = t1476 * t28719;
    let t152770 = t1486 * t193 * t2781 * t152767;
    let t152772 = t33978 * t4255;
    let t152774 = t446 * t10248 * t152772;
    let t152776 = t7584 * t4129;
    let t152779 = t1486 * t193 * t10570 * t152776;
    let t152780 = t6260 * t7021;
    let t152783 = t1486 * t193 * t2781 * t152780;
    let t152788 = t6308 * t193 * t852 * t7611 * t4226;
    (t152767, t152770, t152772, t152774, t152776, t152779, t152780, t152783, t152788)
}
