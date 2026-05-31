//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1199/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1199<F: Float>(t10697: F, t5309: F, t5393: F, t44601: F, t10703: F, t10758: F, t1212: F, t1248: F, t1255: F, t15128: F, t1901: F, t21351: F, t21369: F, t22161: F, t22410: F, t2862: F, t296: F, t319: F, t44445: F, t446: F, t4973: F, t5330: F, t835: F, t840: F, t84628: F, t84630: F, t871: F, t88726: F, t88730: F) -> (F, F, F) {
    let t90873 = t10697 * t5309 * t5393;
    let t90935 = t5309 * t5309;
    let t90936 = t44601 * t90935;
    let t90940 = -F::cast_from(8.0_f64) * t446 * t840 * t15128 * t22410 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t446 * t2862 * t319 * t22161 * t1212 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t446 * t840 * t871 * t22161 * t1248 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t446 * t835 * t1255 * t21369 - t446 * t835 * t319 * t88730 / F::cast_from(9.0_f64) - F::cast_from(40.0_f64) / F::cast_from(81.0_f64) * t446 * t10758 * t1255 * t21351 - F::cast_from(80.0_f64) / F::cast_from(243.0_f64) * t446 * t44445 * t319 * t88726 + F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t84628 + F::cast_from(40.0_f64) / F::cast_from(243.0_f64) * t84630 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t1901 * t10703 * t5330 * t4973 + F::cast_from(8.0_f64) * t446 * t296 * t90936;
    (t90873, t90936, t90940)
}
