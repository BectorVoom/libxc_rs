//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1317/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1317<F: Float>(t2173: F, t32033: F, t6204: F, t6217: F, t25397: F, t32045: F, t5600: F, t110474: F, t113902: F, t113941: F, t113959: F, t114075: F, t118754: F, t118822: F, t118891: F, t118933: F, t32087: F, t32102: F, t33389: F, t33400: F, t33446: F, t33460: F, t9426: F, t9446: F, t9805: F) -> (F, F, F) {
    let t119019 = t6204 * t32033 * t2173 * t6217;
    let t119027 = t5600 * t32045 * t25397;
    let t119037 = 0.17972642500000000001e-2 * t110474 * t118754 + 0.20833333333333333334e-1 * t9446 * t118822 - 0.8041666666666666667e-2 * t33460 * t33400 + 0.62500000000000000002e-1 * t9446 * t118754 - 0.46561250000000000002e-2 * t113959 * t33389 - 0.46561250000000000002e-2 * t32102 * t119019 + 0.23280625000000000001e-2 * t32102 * t118822 + 0.13968375e-1 * t32102 * t118754 - 0.66327777777777777776e-2 * t119027 - 0.120625e-1 * t9426 * t118933 - 0.69444444444444444446e-2 * t114075 * t9805 + 0.69444444444444444446e-2 * t113941 * t33446 - t113902 - 0.13888888888888888889e-1 * t32087 * t118891;
    (t119019, t119027, t119037)
}
