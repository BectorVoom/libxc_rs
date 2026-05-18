//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1183/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1183<F: Float>(t1349: F, t27178: F, t1546: F, t34988: F, t1359: F, t1360: F, t138568: F, t1526: F, t1527: F, t15567: F, t2258: F, t23400: F, t27030: F, t27035: F, t27100: F, t27103: F, t27175: F, t27182: F, t27186: F, t28: F, t2984: F, t2993: F, t3000: F, t3052: F, t32665: F, t32670: F, t3450: F, t34985: F, t34989: F, t5766: F, t5772: F, t5922: F, t6580: F, t6678: F, t8633: F) -> F {
    let t149518 = t1349 * t27178;
    let t149524 = t1349 * t1546 * t34988;
    let t149549 = t15567 * t2258 * t1359 * t2993 / F::new(6.0) - t15567 * t8633 * t1359 * t2984 / F::new(9.0) + t1349 * t27175 / F::new(3.0) + t5766 * t6678 / F::new(3.0) + t1349 * t27182 / F::new(3.0) + t1349 * t27186 / F::new(3.0) - t149518 / F::new(9.0) + t6580 * t5922 / F::new(3.0) - t138568 / F::new(54.0) - t149524 / F::new(54.0) - t1526 * t1527 * t27035 / F::new(12.0) - t1526 * t1527 * t27030 / F::new(12.0) - t34985 * t32670 / F::new(6.0) - t1349 * t28 * t23400 * t3450 + t1349 * t3000 * t1360 * t3052 / F::new(9.0) + t6580 * t32665 / F::new(18.0) + t5766 * t34989 / F::new(18.0) - t5772 * t27100 / F::new(9.0) + t5772 * t27103 / F::new(27.0);
    t149549
}
