//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1143/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1143<F: Float>(t1881: F, t8015: F, t2167: F, t6290: F, t1884: F, t2132: F, t2233: F, t8130: F, t8136: F, t2273: F, t6888: F, t1885: F, t8255: F) -> (F, F, F, F, F, F) {
    let t28891 = t1881 * t8015;
    let t29238 = t6290 * t2167;
    let t29247 = t1884 * t2132;
    let t29248 = t2233 * t29247;
    let t29249 = t29248 / F::cast_from(8.0_f64);
    let t29250 = t8130 * t8136;
    let t29251 = t29250 / F::cast_from(8.0_f64);
    let t29252 = t6888 * t2273;
    let t29253 = t29252 / F::cast_from(8.0_f64);
    let t29254 = t1885 * t8255;
    (t28891, t29238, t29249, t29251, t29253, t29254)
}
