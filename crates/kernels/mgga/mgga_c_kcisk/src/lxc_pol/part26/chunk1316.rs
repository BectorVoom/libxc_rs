//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1316/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1316<F: Float>(t110225: F, t118764: F, t6183: F, t34798: F, t3973: F, t9446: F, t113997: F, t114199: F, t114205: F, t114231: F, t118759: F, t118962: F, t25342: F, t25406: F, t32008: F, t32022: F, t32087: F, t33408: F, t33415: F, t34799: F, t3937: F, t6175: F, t9805: F) -> (F,) {
    let t118984 = t6183 * t110225 * t118764;
    let t118996 = t9446 * t3973 * t34798;
    let t119006 = 0.69444444444444444446e-2 * t32087 * t6183 * t33408 * t25406 + 0.13402777777777777778e-2 * t32008 * t118962 - 0.46296296296296296297e-2 * t32087 * t6175 * t33415 * t25406 - 0.69444444444444444446e-2 * t32087 * t3937 * t33408 * t118759 - 0.69444444444444444446e-2 * t32087 * t118984 - 0.20833333333333333334e-1 * t32087 * t6183 * t33415 * t25342 - 0.26805555555555555556e-2 * t32008 * t118984 + 0.92592592592592592595e-2 * t32022 * t34799 - 0.11574074074074074074e-2 * t118996 + 0.18518518518518518519e-1 * t113997 * t9805 - 0.69444444444444444446e-2 * t114205 * t9805 - 0.69444444444444444446e-2 * t114231 * t9805 - 0.69444444444444444446e-2 * t114199 * t9805;
    (t119006,)
}
