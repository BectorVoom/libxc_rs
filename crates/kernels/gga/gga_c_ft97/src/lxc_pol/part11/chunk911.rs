//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 911/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk911<F: Float>(t3281: F, t454: F, t1822: F, t8232: F, t1893: F, t110: F, t1825: F, t1866: F, t1901: F, t38079: F, t38103: F, t38254: F, t38711: F, t38732: F, t432: F, t446: F, t447: F, t452: F, t488: F, t492: F, t8183: F, t8549: F, t8558: F, t8590: F) -> F {
    let t38734 = t3281 * t454;
    let t38740 = t8232 * t1822;
    let t38742 = t8232 * t1893;
    let t38744 = F::cast_from(4.0_f64) * t446 * t452 * t1825 * t8549 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t1901 * t38711 * t8558 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t446 * t1866 * t110 * t38079 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t447 * t110 * t38103 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t446 * t452 * t488 * t8183 * t492 - t446 * t452 * t110 * t38254 / F::cast_from(3.0_f64) + F::cast_from(112.0_f64) / F::cast_from(81.0_f64) * t38732 + F::cast_from(112.0_f64) / F::cast_from(81.0_f64) * t38734 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t446 * t452 * t8590 * t432 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t38740 - F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t38742;
    t38744
}
