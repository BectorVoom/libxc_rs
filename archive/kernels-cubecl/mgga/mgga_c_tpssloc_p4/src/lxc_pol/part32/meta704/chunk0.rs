//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2202/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2202<F: Float>(t24996: F, t97890: F, t28860: F, t6876: F, t1307: F, t6324: F, t22574: F, t26162: F, t28835: F, t28830: F, t24995: F, t8643: F) -> (F, F, F, F, F) {
    let t97892 = F::cast_from(12.0_f64) * t97890 * t24996;
    let t97893 = t6876 * t28860;
    let t97894 = t6324 * t1307;
    let t97897 = F::cast_from(6.0_f64) * t22574 * t26162 * t97894;
    let t97899 = F::cast_from(3.0_f64) * t6876 * t28835;
    let t97902 = t28830 * t1307;
    let t97905 = F::cast_from(12.0_f64) * t24995 * t8643 * t97902;
    (t97892, t97893, t97897, t97899, t97905)
}
