//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 971/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk971<F: Float>(t1724: F, t1866: F, t1620: F, t1627: F, t17883: F, t17911: F, t1792: F, t17931: F, t17964: F, t17989: F, t1803: F, t1809: F, t1817: F, t185: F, t186: F, t211: F, t422: F, t4903: F, t5048: F, t5146: F, t5162: F, t5352: F, t5467: F, t5470: F, t5524: F, t617: F, t626: F, t650: F, t663: F, t7011: F) -> F {
    let t17996 = t1724 * t1724;
    let t18001 = t1866 * t1866;
    let t18008 = F::cast_from(32.0_f64) / F::cast_from(45.0_f64) * t1620 * t1809 * t5048 * t617 - F::cast_from(128.0_f64) / F::cast_from(45.0_f64) * t17883 - F::cast_from(32.0_f64) / F::cast_from(15.0_f64) * t5467 * t1817 - F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t5470 * t1817 - F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t1627 * t5146 - F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t1627 * t5524 + F::cast_from(32.0_f64) / F::cast_from(45.0_f64) * t1620 * t1809 * t5162 * t626 * t422 + F::cast_from(32.0_f64) / F::cast_from(15.0_f64) * t7011 * t4903 - F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t211 * t186 * t650 * (t17911 + t17931 + t17964 + t17989) + F::cast_from(4.0_f64) / F::cast_from(5.0_f64) * t211 * t186 * t1792 * t17996 + F::cast_from(4.0_f64) / F::cast_from(5.0_f64) * t185 * t186 * t1803 * t18001 - F::cast_from(16.0_f64) / F::cast_from(5.0_f64) * t5352 * t663;
    t18008
}
