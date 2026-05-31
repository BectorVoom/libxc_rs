//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1057/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1057<F: Float>(t2035: F, t32626: F, t7313: F, t8698: F, t531: F, t8713: F, t7238: F, t2014: F, t2107: F, t32113: F, t7235: F, t8718: F) -> (F, F, F, F, F, F, F, F) {
    let t32627 = t32626 * t2035;
    let t32628 = t8698 * t7313;
    let t32629 = t531 * t8713;
    let t32630 = t32629 * t7238;
    let t32632 = F::cast_from(3.0_f64) * t2014 * t32630;
    let t32633 = t2107 * t32113;
    let t32634 = t2014 * t32633;
    let t32635 = t7235 * t8718;
    (t32627, t32628, t32629, t32630, t32632, t32633, t32634, t32635)
}
