//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 1045/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk1045<F: Float>(t10148: F, t1356: F, t2379: F, t28295: F, t289: F, t36332: F, t4041: F, t40655: F, t43492: F, t46379: F, t47162: F, t47167: F, t47173: F, t47175: F, t47178: F, t47180: F, t47182: F, t47188: F, t47190: F, t47196: F, t5019: F, t9855: F) -> F {
    let t47198 = F::cast_from(0.25538759935978703638e-4_f64) * t47162 - F::cast_from(0.25538759935978703638e-4_f64) * t47167 + F::cast_from(0.39914139006212695214e-1_f64) * t1356 * t46379 + F::cast_from(0.11974241701863808564e0_f64) * t28295 * t2379 + F::cast_from(0.17025839957319135759e-4_f64) * t47173 - t43492 - F::cast_from(0.11974241701863808564e0_f64) * t47175 - F::new(0.2363e1) * t36332 + F::cast_from(0.19863479950205658386e-4_f64) * t47178 + F::cast_from(0.19863479950205658386e-4_f64) * t47180 + F::cast_from(0.59590439850616975155e-4_f64) * t47182 + F::cast_from(0.59871208509319042821e-1_f64) * t4041 * t10148 - F::cast_from(0.23948483403727617128e0_f64) * t5019 * t9855 + F::cast_from(0.79828278012425390427e-1_f64) * t47188 - F::new(0.4726e1) * t289 * t47190 - t40655 - F::cast_from(0.51077519871957407276e-4_f64) * t47196;
    t47198
}
