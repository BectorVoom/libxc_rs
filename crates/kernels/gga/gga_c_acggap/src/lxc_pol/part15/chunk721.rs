//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 721/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk721<F: Float>(t390: F, t7746: F, t7458: F, t7487: F, t1980: F, t1967: F, t2087: F, t2092: F, t1988: F, t7476: F, t7483: F, t1973: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7747 = t7746 * t390;
    let t7753 = t7458 * t7487;
    let t7754 = t1980 * t7753;
    let t7755 = F::cast_from(0.28582678745379824648e-3_f64) * t7754;
    let t7756 = t1967 * t2087;
    let t7758 = t1967 * t2092;
    let t7759 = F::cast_from(0.25724410870841842184e-2_f64) * t7758;
    let t7760 = t1988 * t2087;
    let t7761 = F::cast_from(0.10718504529517434243e-2_f64) * t7760;
    let t7770 = t7476 * t7483;
    let t7771 = t1980 * t7770;
    let t7772 = F::cast_from(0.7145669686344956162e-3_f64) * t7771;
    let t7773 = t1967 * t1973;
    (t7747, t7753, t7755, t7756, t7759, t7761, t7770, t7772, t7773)
}
