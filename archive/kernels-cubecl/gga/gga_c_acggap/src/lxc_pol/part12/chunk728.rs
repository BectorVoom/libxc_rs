//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 728/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk728<F: Float>(t390: F, t7746: F, t1020: F, t2001: F, t1029: F, t7458: F, t7487: F, t1980: F, t1967: F, t2087: F, t2092: F, t1988: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t7747 = t7746 * t390;
    let t7749 = t2001 * t1020;
    let t7751 = t2001 * t1029;
    let t7753 = t7458 * t7487;
    let t7754 = t1980 * t7753;
    let t7755 = F::cast_from(0.28582678745379824648e-3_f64) * t7754;
    let t7756 = t1967 * t2087;
    let t7758 = t1967 * t2092;
    let t7759 = F::cast_from(0.25724410870841842184e-2_f64) * t7758;
    let t7760 = t1988 * t2087;
    (t7747, t7749, t7751, t7753, t7754, t7755, t7756, t7758, t7759, t7760)
}
