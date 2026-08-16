//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2134/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2134<F: Float>(t1873: F, t19289: F, t652: F, t1983: F, t20085: F, t6996: F, t28827: F, t6876: F, t7684: F, t8944: F, t26164: F, t24995: F, t75203: F, t8643: F) -> (F, F, F, F, F) {
    let t96789 = F::cast_from(2.0_f64) * t652 * t19289 * t1873;
    let t96792 = F::cast_from(2.0_f64) * t1983 * t6996 * t20085;
    let t96796 = F::cast_from(6.0_f64) * t6876 * t28827;
    let t96797 = t7684 * t8944;
    let t96799 = F::cast_from(4.0_f64) * t96797 * t26164;
    let t96802 = F::cast_from(6.0_f64) * t24995 * t8643 * t75203;
    (t96789, t96792, t96796, t96799, t96802)
}
