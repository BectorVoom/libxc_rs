//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1077/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1077<F: Float>(t44: F, t48: F, t4902: F, t4918: F, t51: F, t53: F, t1384: F, t4854: F, t234: F, t4997: F, t1381: F, t1409: F, t4859: F, t4862: F, t1520: F, t5018: F) -> (F, F, F, F, F, F) {
    let t19347 = 1.0 / t48 / t4902 / t44;
    let t19363 = 1.0 / t53 / t4918 / t51;
    let t19385 = t1384 * t4854;
    let t19388 = 0.69263436422725855036e2 * t234 * t4997 * t19385;
    let t19394 = 0.61524113149298439947e4 * t234 * t4859 * t1381 * t4862 * t1409;
    let t19400 = t1520 * t5018;
    (t19347, t19363, t19385, t19388, t19394, t19400)
}
