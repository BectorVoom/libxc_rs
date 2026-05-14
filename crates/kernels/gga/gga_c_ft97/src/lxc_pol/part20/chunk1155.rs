//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1155/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1155<F: Float>(t28361: F, t46862: F, t28405: F, t8392: F, t28401: F, t6914: F, t8232: F, t28388: F, t28346: F, t10157: F, t107845: F, t108218: F, t11593: F, t13859: F, t13897: F, t14133: F, t14163: F, t14175: F, t14192: F, t1456: F, t1901: F, t2409: F, t242: F, t24569: F, t24793: F, t27742: F, t446: F, t6930: F, t729: F, t773: F, t97777: F, t97791: F) -> (F,) {
    let t110702 = t46862 * t28361;
    let t110713 = 4.0 / 81.0 * t8392 * t28405;
    let t110718 = 4.0 / 27.0 * t8392 * t28401;
    let t110719 = t8232 * t6914;
    let t110733 = 4.0 / 27.0 * t8392 * t28388;
    let t110735 = 4.0 / 81.0 * t8392 * t28346;
    let t110743 = 22.0 / 27.0 * t110702 + 2.0 / 27.0 * t97791 + 2.0 / 3.0 * t446 * t242 * t107845 - 2.0 / 3.0 * t446 * t729 * t773 * t27742 + t110713 - 2.0 / 9.0 * t1901 * t24793 * t13859 - t110718 - 4.0 / 27.0 * t110719 - 2.0 * t446 * t10157 * t1456 * t14133 - 2.0 / 9.0 * t1901 * t97777 * t14192 + 8.0 / 9.0 * t11593 * t14175 * t24569 * t13897 + t110733 - t110735 + 4.0 / 9.0 * t1901 * t14175 * t6930 * t2409 - 4.0 / 9.0 * t1901 * t14163 * t108218;
    (t110743,)
}
