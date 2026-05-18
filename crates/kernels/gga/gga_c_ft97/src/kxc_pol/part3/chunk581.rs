//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 581/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk581<F: Float>(t4518: F, t582: F, t4522: F, t2112: F, t24: F, t4668: F, t4714: F, t586: F, t2092: F, t3497: F, t3513: F, t462: F, t4759: F, t4762: F, t92: F) -> (F, F, F, F, F) {
    let t4765 = t582 * t4518;
    let t4768 = t582 * t4522;
    let t4772 = t24 * t2112 * t4668;
    let t4776 = t24 * t586 * t4714;
    let t4778 = t2092 + F::new(2.0) / F::new(9.0) * t3497 + F::new(2.0) / F::new(3.0) * t3513 - F::new(2.0) / F::new(9.0) * t462 * t4759 + F::new(2.0) / F::new(3.0) * t462 * t4762 + F::new(2.0) / F::new(3.0) * t462 * t4765 - t462 * t4768 / F::new(3.0) + F::new(2.0) * t92 * t4772 - t92 * t4776;
    (t4765, t4768, t4772, t4776, t4778)
}
