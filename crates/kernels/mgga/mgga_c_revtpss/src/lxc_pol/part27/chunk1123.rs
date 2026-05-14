//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1123/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1123<F: Float>(t4004: F, t676: F, t25880: F, t94763: F, t25894: F, t94762: F, t25877: F, t94382: F, t94590: F, t25950: F, t26050: F, t25304: F, t25949: F, t25946: F, t25878: F, t94661: F) -> (F, F, F, F, F, F) {
    let t94764 = t676 * t4004;
    let t94765 = t25880 * t94764;
    let t94766 = t94763 * t94765;
    let t94768 = t25894 * t94762;
    let t94769 = t94768 * t94765;
    let t94771 = t94382 * t25877;
    let t94772 = t94771 * t94590;
    let t94774 = t25950 * t26050;
    let t94776 = t25304 * t25949;
    let t94777 = t94776 * t25946;
    let t94779 = t25878 * t94661;
    (t94766, t94769, t94772, t94774, t94777, t94779)
}
