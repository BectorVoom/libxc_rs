//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1138/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1138<F: Float>(t30663: F, t6555: F, t6552: F, t6572: F, t1880: F, t23237: F, t8335: F, t6547: F, t8357: F, t1902: F, t234: F, t776: F) -> (F, F, F, F, F, F, F, F, F) {
    let t30664 = t30663 * t6555;
    let t30666 = F::cast_from(0.3289868133696452873e-1_f64) * t6552 * t30664;
    let t30667 = t30663 * t6572;
    let t30669 = F::cast_from(0.16449340668482264365e-1_f64) * t1880 * t30667;
    let t30671 = t23237 * t8335;
    let t30673 = F::cast_from(0.16449340668482264365e-1_f64) * t1880 * t30671;
    let t30675 = F::cast_from(0.38381794893125283518e-1_f64) * t6547 * t8357;
    let t30676 = t234 * t1902;
    let t30677 = t30676 * t776;
    (t30664, t30666, t30667, t30669, t30671, t30673, t30675, t30676, t30677)
}
