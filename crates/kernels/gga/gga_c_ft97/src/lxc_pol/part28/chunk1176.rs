//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1176/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1176<F: Float>(t1389: F, t6615: F, t1349: F, t138662: F, t148210: F, t148225: F, t148880: F, t148977: F, t1969: F, t26785: F, t28: F, t32714: F, t32879: F, t3450: F, t34963: F, t35033: F, t35196: F, t3588: F, t379: F, t5766: F, t5772: F, t5968: F, t609: F, t6708: F, t7340: F, t9432: F, t9439: F) -> F {
    let t149296 = t6615 * t1389;
    let t149301 = t5766 * t34963 / F::cast_from(6.0_f64) - F::cast_from(24.0_f64) * t9439 * t35033 * t609 - F::cast_from(12.0_f64) * t9439 * t35196 * t609 + t138662 - F::cast_from(24.0_f64) * t9439 * t6708 * t5968 + F::cast_from(4.0_f64) * t148210 + F::cast_from(4.0_f64) * t148880 + t1349 * t28 * t7340 * t3588 / F::cast_from(6.0_f64) + F::cast_from(8.0_f64) * t148977 + F::cast_from(4.0_f64) * t148225 + F::cast_from(2.0_f64) * t5772 * t9432 * t32879 * t3450 - t32714 * t26785 / F::cast_from(18.0_f64) - t5772 * t1969 * t149296 * t379 / F::cast_from(9.0_f64);
    t149301
}
