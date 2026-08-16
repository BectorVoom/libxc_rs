//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1082/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1082<F: Float>(t1063: F, t13829: F, t448: F, t13732: F, t6313: F, t42730: F, t42733: F, t42737: F, t42739: F, t42742: F, t42743: F, t42745: F, t42748: F, t42751: F) -> F {
    let t46979 = F::cast_from(0.28455006635676149599e-1_f64) * t1063 * t13829 * t448;
    let t46980 = t6313 * t13732;
    let t46986 = -t42730 + t42733 - t46979 - F::cast_from(0.1138200265427045984e0_f64) * t46980 - t42737 + t42739 + t42742 + F::cast_from(0.7588001769513639893e-1_f64) * t42743 + F::cast_from(0.11856252764865062333e-2_f64) * t42745 + F::cast_from(0.11856252764865062333e-2_f64) * t42748 + F::cast_from(0.85365019907028448797e-1_f64) * t42751;
    t46986
}
