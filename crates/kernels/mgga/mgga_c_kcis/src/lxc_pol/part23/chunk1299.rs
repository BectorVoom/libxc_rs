//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1299/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1299<F: Float>(t1307: F, t28736: F, t95024: F, t2109: F, t27614: F, t4463: F, t6176: F, t30424: F, t4457: F, t18227: F, t7969: F, t27567: F, t7968: F, t7978: F, t98411: F, t98414: F, t98417: F, t98449: F, t98452: F, t99322: F) -> (F, F, F, F) {
    let t99341 = t95024 * t28736 * t1307;
    let t99348 = t6176 * t27614 * t2109 * t4463;
    let t99353 = t6176 * t30424 * t2109 * t4457;
    let t99360 = t6176 * t7969 * t18227;
    let t99367 = -F::cast_from(0.30918233506944444444e-4_f64) * t27567 * t99341 - F::cast_from(0.92754700520833333333e-4_f64) * t27567 * t99322 - F::cast_from(0.34752604166666666667e-3_f64) * t7978 * t99348 + F::cast_from(0.92754700520833333334e-4_f64) * t7968 * t99353 + F::cast_from(0.61905925925925925924e-2_f64) * t98411 - F::cast_from(0.41270617283950617282e-2_f64) * t98414 + F::cast_from(0.15476481481481481481e-2_f64) * t98417 + F::cast_from(0.46377350260416666667e-4_f64) * t7968 * t99360 + F::cast_from(0.34752604166666666667e-3_f64) * t7978 * t99360 + F::cast_from(0.11607361111111111111e-2_f64) * t98449 - F::cast_from(0.46429444444444444444e-2_f64) * t98452;
    (t99341, t99348, t99353, t99367)
}
