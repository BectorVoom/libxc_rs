//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 556/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk556<F: Float>(t2917: F, t242: F, t1060: F, t250: F, t253: F, t659: F, t946: F, t251: F, t992: F) -> (F, F, F, F, F, F, F) {
    let t2947 = F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t2917;
    let t2955 = F::cast_from(0.39862222222222222223e0_f64) * t2917;
    let t2960 = F::cast_from(1.0_f64)/F::sqrt(t242);
    let t2966 = t250 * t1060 * t253;
    let t2967 = F::cast_from(0.13692777777777777778e0_f64) * t2966;
    let t2968 = t659 * t946;
    let t2970 = t251 * t992;
    (t2947, t2955, t2960, t2966, t2967, t2968, t2970)
}
