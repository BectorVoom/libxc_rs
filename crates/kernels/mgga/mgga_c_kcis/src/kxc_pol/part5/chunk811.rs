//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 811/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk811<F: Float>(t3005: F, t6406: F, t971: F, t3013: F, t3020: F, t4612: F, t4706: F, t6328: F, t6332: F, t6336: F, t6341: F, t6343: F, t6375: F, t6377: F, t6381: F, t6384: F, t6387: F) -> (F, F) {
    let t6408 = t3005 * t6406 * t971;
    let t6423 = -F::new(0.1294625e1) * t6341 + F::new(0.258925e1) * t6343 + t3013 + F::new(0.20128333333333333334e0) * t4612 - F::new(0.20128333333333333333e0) * t6328 + F::new(0.60385e0) * t6332 - F::new(0.301925e0) * t6336 + F::new(0.82524375e-1) * t6375 + F::new(0.16504875e0) * t6377 + t3020 + F::new(0.11038e0) * t4706 - F::new(0.27595e-1) * t6381 + F::new(0.16557e0) * t6384 - F::new(0.82785e-1) * t6387;
    (t6408, t6423)
}
