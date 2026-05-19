//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 907/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk907<F: Float>(t3873: F, t6964: F, t1319: F, t11455: F, t11608: F, t11609: F, t21268: F, t21270: F, t21273: F, t21275: F, t21278: F, t21281: F, t21283: F) -> (F, F) {
    let t21285 = t3873 * t6964;
    let t21286 = t21285 * t1319;
    let t21288 = -F::cast_from(0.91285185185185185187e-1_f64) * t11455 - t11608 - t11609 + F::new(0.1898925e1) * t21268 + F::new(0.3071625e0) * t21270 + F::cast_from(0.142419375e1_f64) * t21273 - F::new(0.1898925e1) * t21275 - F::new(0.9494625e0) * t21278 - F::new(0.76790625e-1) * t21281 + F::new(0.3071625e0) * t21283 + F::new(0.15358125e0) * t21286;
    (t21286, t21288)
}
