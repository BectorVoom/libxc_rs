//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 956/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk956<F: Float>(t1775: F, t20830: F, t20813: F, t20799: F, t20802: F, t20804: F, t1882: F, t20565: F, t20561: F, t20557: F, t20553: F, t20569: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t78188 = t1775 * t20830;
    let t78242 = t1775 * t20813;
    let t78247 = t1775 * t20799;
    let t78249 = t1775 * t20802;
    let t78251 = t1775 * t20804;
    let t78362 = t1882 * t20565;
    let t78364 = t1882 * t20561;
    let t78366 = t1882 * t20557;
    let t78368 = t1882 * t20553;
    let t78396 = t1882 * t20569;
    (t78188, t78242, t78247, t78249, t78251, t78362, t78364, t78366, t78368, t78396)
}
