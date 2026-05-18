//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1374/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1374<F: Float>(t5458: F, t5469: F, t33724: F, t33730: F, t43414: F, t44193: F, t44198: F, t58348: F, t58352: F, t58356: F, t58360: F, t58363: F, t58367: F) -> (F, F, F) {
    let t58464 = t5458 * t5458;
    let t58470 = t5469 * t5469;
    let t58487 = -F::new(0.17481481481481481482e3) * t44193 + F::new(0.10488888888888888889e4) * t44198 - F::new(0.12922962962962962963e4) * t43414 + F::new(0.30153580246913580247e4) * t33724 + F::new(0.93234567901234567903e3) * t33730 + F::new(0.58153333333333333332e4) * t58348 + F::new(0.94399999999999999998e3) * t58352 - F::new(0.20977777777777777778e3) * t58356 - F::new(2832.0) * t58360 + F::new(0.62933333333333333332e3) * t58363 - F::new(0.10488888888888888889e3) * t58367;
    (t58464, t58470, t58487)
}
