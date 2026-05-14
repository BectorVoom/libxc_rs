//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1390/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1390<F: Float>(t38953: F, t6696: F, t1882: F, t27260: F, t6705: F, t8232: F, t2097: F, t5935: F, t105338: F, t106189: F, t11593: F, t12561: F, t12715: F, t12729: F, t12733: F, t12968: F, t1391: F, t144: F, t1901: F, t2142: F, t23443: F, t26897: F, t26918: F, t3455: F, t446: F, t574: F, t9099: F, t95813: F, t96239: F, t96244: F, t96251: F, t96269: F) -> (F,) {
    let t107614 = t38953 * t6696;
    let t107621 = 2.0 / 9.0 * t1882 * t27260;
    let t107625 = t8232 * t6705;
    let t107627 = t2097 * t5935;
    let t107645 = t96244 - t446 * t574 * t1391 * t12561 / 3.0 + 2.0 / 27.0 * t96251 + 4.0 / 9.0 * t11593 * t9099 * t26918 + 4.0 / 81.0 * t107614 + 2.0 / 3.0 * t446 * t574 * t2142 * t26897 + t107621 - t446 * t144 * t106189 / 3.0 - 4.0 / 27.0 * t107625 + 4.0 / 27.0 * t1901 * t107627 * t12715 - 4.0 / 3.0 * t1901 * t12968 * t95813 * t3455 + t1901 * t23443 * t12729 / 9.0 + 2.0 / 27.0 * t1901 * t96239 * t12733 - 2.0 / 27.0 * t96269 + 4.0 / 3.0 * t446 * t144 * t105338;
    (t107645,)
}
