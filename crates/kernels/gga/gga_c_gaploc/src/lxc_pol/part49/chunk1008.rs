//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1008/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1008<F: Float>(t41984: F, t41987: F, t41989: F, t41991: F, t41992: F, t41996: F, t42001: F, t42005: F, t42008: F, t42015: F, t42018: F, t42022: F, t40320: F, t13826: F, t1580: F, t46952: F, t568: F, t597: F, t600: F) -> (F, F, F, F) {
    let t48010 = t41984 - t41987 - t41989 + t41991 + t41992 - t41996 - 0.29792074959875355558e-1 * t42001 + t42005 + t42008 - 0.69017266717057349418e1 * t42015 - t42018 - t42022;
    let t48011 = 0.72851559312449424385e1 * t40320;
    let t48013 = 0.23005755572352449806e1 * t1580 * t13826;
    let t48017 = 0.23005755572352449806e1 * t597 * t568 * t600 * t46952;
    (t48010, t48011, t48013, t48017)
}
