//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 1045/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk1045(t10148: f64, t1356: f64, t2379: f64, t28295: f64, t289: f64, t36332: f64, t4041: f64, t40655: f64, t43492: f64, t46379: f64, t47162: f64, t47167: f64, t47173: f64, t47175: f64, t47178: f64, t47180: f64, t47182: f64, t47188: f64, t47190: f64, t47196: f64, t5019: f64, t9855: f64) -> f64 {
    let t47198 = 0.25538759935978703638e-4_f64 * t47162 - 0.25538759935978703638e-4_f64 * t47167 + 0.39914139006212695214e-1_f64 * t1356 * t46379 + 0.11974241701863808564e0_f64 * t28295 * t2379 + 0.17025839957319135759e-4_f64 * t47173 - t43492 - 0.11974241701863808564e0_f64 * t47175 - 0.2363e1_f64 * t36332 + 0.19863479950205658386e-4_f64 * t47178 + 0.19863479950205658386e-4_f64 * t47180 + 0.59590439850616975155e-4_f64 * t47182 + 0.59871208509319042821e-1_f64 * t4041 * t10148 - 0.23948483403727617128e0_f64 * t5019 * t9855 + 0.79828278012425390427e-1_f64 * t47188 - 0.4726e1_f64 * t289 * t47190 - t40655 - 0.51077519871957407276e-4_f64 * t47196;
    t47198
}
