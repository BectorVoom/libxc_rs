//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1366/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1366<F: Float>(t1864: F, t415: F, t8874: F, t112586: F, t23309: F, t5182: F, t1785: F, t7715: F, t10802: F, t116123: F, t116651: F, t121323: F, t121440: F, t121443: F, t121446: F, t121454: F, t121457: F, t121460: F, t32948: F, t33031: F, t34016: F, t34125: F, t34133: F, t35097: F, t9672: F, t9936: F) -> (F, F, F, F) {
    let t121463 = t415 * t1864 * t8874;
    let t121468 = t5182 * t112586 * t23309;
    let t121471 = t7715 * t1785;
    let t121476 = 0.99491666666666666664e-2 * t121440 - 0.69444444444444444447e-2 * t121443 - 0.11574074074074074074e-2 * t121446 + 0.18518518518518518519e-1 * t116123 * t9936 - 0.37037037037037037038e-1 * t34125 * t34133 - 0.8041666666666666667e-2 * t32948 * t35097 - 0.16581944444444444444e-2 * t121454 - 0.77602083333333333337e-3 * t121457 + 0.16581944444444444444e-2 * t121460 - 0.55273148148148148147e-3 * t121463 - 0.55555555555555555558e-1 * t121323 * t9672 - 0.22109259259259259259e-2 * t121468 + 0.23148148148148148149e-2 * t116651 + 0.46296296296296296297e-2 * t33031 * t10802 * t34016 * t121471;
    (t121463, t121468, t121471, t121476)
}
