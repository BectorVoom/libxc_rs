//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1106/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1106<F: Float>(t4459: F, t6155: F, t19561: F, t816: F, t825: F, t2373: F, t6745: F, t2365: F, t6158: F, t6164: F, t822: F, t1452: F, t19750: F, t19772: F, t19778: F, t19791: F, t2118: F, t2358: F, t2362: F, t2382: F, t2384: F, t2392: F, t3079: F, t328: F, t4385: F, t6106: F, t6145: F, t6151: F, t6160: F, t8606: F) -> (F, F) {
    let t19794 = t6155 * t4459;
    let t19803 = t19561 * t816;
    let t19804 = t19803 * t825;
    let t19808 = t6745 * t2373;
    let t19810 = t6158 * t2365;
    let t19812 = t822 * t19810 * t6164;
    let t19816 = t4385 * t19772 / F::cast_from(8.0_f64) - F::cast_from(7.0_f64) / F::cast_from(24.0_f64) * t19778 + t2382 * t2118 * t1452 * t328 * t3079 / F::cast_from(24.0_f64) + t2382 * t2118 * t19750 * t8606 / F::cast_from(8.0_f64) + t6160 * t19791 / F::cast_from(12.0_f64) + F::cast_from(7.0_f64) / F::cast_from(24.0_f64) * t19794 - t6106 * t2358 * t2362 / F::cast_from(32.0_f64) + t2384 * t6145 / F::cast_from(8.0_f64) + F::cast_from(3.0_f64) / F::cast_from(8.0_f64) * t2384 * t6151 + F::cast_from(11.0_f64) / F::cast_from(96.0_f64) * t2382 * t19804 * t6164 + F::cast_from(7.0_f64) / F::cast_from(12.0_f64) * t19808 - F::cast_from(7.0_f64) / F::cast_from(36.0_f64) * t19812 + F::cast_from(3.0_f64) / F::cast_from(8.0_f64) * t2392 * t6151;
    (t19803, t19816)
}
