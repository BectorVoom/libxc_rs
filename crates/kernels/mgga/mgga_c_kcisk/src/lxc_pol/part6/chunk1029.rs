//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 1029/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk1029<F: Float>(t20820: F, t7877: F, t30158: F, t425: F, t2083: F, t7764: F, t13263: F, t19100: F, t25590: F, t25601: F, t25609: F, t30569: F, t30572: F, t30592: F, t30595: F, t30599: F, t30603: F, t30608: F, t30610: F, t30617: F) -> (F, F, F, F) {
    let t30900 = t20820 * t7877;
    let t30909 = t425 * t30158;
    let t30916 = t2083 * t7764;
    let t30938 = F::new(0.14865e-1) * t30617 - F::new(0.2973e-1) * t30610 + F::new(0.1982e-1) * t30608 - t13263 - F::cast_from(0.55033333333333333332e-2_f64) * t19100 + F::cast_from(0.27516666666666666666e-2_f64) * t25590 - F::cast_from(0.82549999999999999999e-2_f64) * t25601 + F::cast_from(0.41274999999999999999e-2_f64) * t25609 - F::cast_from(0.45861111111111111112e-2_f64) * t30592 + F::new(0.1651e-1) * t30595 - F::cast_from(0.82550000000000000001e-2_f64) * t30569 - F::new(0.24765e-1) * t30599 + F::new(0.24765e-1) * t30572 - F::new(0.41275e-2) * t30603;
    (t30900, t30909, t30916, t30938)
}
