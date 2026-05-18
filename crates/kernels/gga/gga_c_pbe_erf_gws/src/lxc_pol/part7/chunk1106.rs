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
    let t19816 = t4385 * t19772 / F::new(8.0) - F::new(7.0) / F::new(24.0) * t19778 + t2382 * t2118 * t1452 * t328 * t3079 / F::new(24.0) + t2382 * t2118 * t19750 * t8606 / F::new(8.0) + t6160 * t19791 / F::new(12.0) + F::new(7.0) / F::new(24.0) * t19794 - t6106 * t2358 * t2362 / F::new(32.0) + t2384 * t6145 / F::new(8.0) + F::new(3.0) / F::new(8.0) * t2384 * t6151 + F::new(11.0) / F::new(96.0) * t2382 * t19804 * t6164 + F::new(7.0) / F::new(12.0) * t19808 - F::new(7.0) / F::new(36.0) * t19812 + F::new(3.0) / F::new(8.0) * t2392 * t6151;
    (t19803, t19816)
}
