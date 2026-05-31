//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1087/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1087<F: Float>(t1060: F, t12680: F, t12709: F, t12968: F, t12969: F, t17164: F, t1901: F, t20655: F, t20723: F, t20763: F, t20858: F, t20926: F, t2210: F, t2221: F, t2992: F, t41251: F, t446: F, t574: F, t63746: F, t63795: F, t76567: F, t77602: F, t77713: F, t9144: F, t925: F) -> F {
    let t87589 = F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t63746 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t1901 * t17164 * t20858 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t1901 * t2221 * t76567 * t925 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t1901 * t9144 * t20723 * t925 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t1901 * t12709 * t2992 * t20763 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t446 * t574 * t1060 * t20655 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t1901 * t2210 * t77602 * t925 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t1901 * t12680 * t20926 - F::cast_from(8.0_f64) * t1901 * t12968 * t12969 * t20723 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t1901 * t41251 * t77713 * t925 - F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t63795;
    t87589
}
