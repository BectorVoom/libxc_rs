//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 868/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk868<F: Float>(t4035: F, t7122: F, t3833: F, t5469: F, t6939: F, t6942: F, t6946: F, t6958: F, t6965: F, t1410: F, t1897: F, t3821: F, t456: F, t5510: F, t6957: F, t6964: F) -> (F, F, F) {
    let t7123 = t4035 * t7122;
    let t7138 = -F::new(0.991e-2) * t6958 + F::new(0.1982e-1) * t6965 + t3833 + F::cast_from(0.27516666666666666666e-2_f64) * t5469 - F::cast_from(0.27516666666666666667e-2_f64) * t6939 + F::new(0.8255e-2) * t6942 - F::new(0.41275e-2) * t6946;
    let t7141 = -t3821 * t6957 / F::new(8.0) + t5510 * t1897 / F::new(2.0) + t1410 * t6964 / F::new(4.0) + t456 * t7138 / F::new(2.0);
    (t7123, t7138, t7141)
}
