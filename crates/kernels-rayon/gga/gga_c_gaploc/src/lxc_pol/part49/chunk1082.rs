//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1082/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1082(t1063: f64, t13829: f64, t448: f64, t13732: f64, t6313: f64, t42730: f64, t42733: f64, t42737: f64, t42739: f64, t42742: f64, t42743: f64, t42745: f64, t42748: f64, t42751: f64) -> f64 {
    let t46979 = 0.28455006635676149599e-1_f64 * t1063 * t13829 * t448;
    let t46980 = t6313 * t13732;
    let t46986 = -t42730 + t42733 - t46979 - 0.1138200265427045984e0_f64 * t46980 - t42737 + t42739 + t42742 + 0.7588001769513639893e-1_f64 * t42743 + 0.11856252764865062333e-2_f64 * t42745 + 0.11856252764865062333e-2_f64 * t42748 + 0.85365019907028448797e-1_f64 * t42751;
    t46986
}
