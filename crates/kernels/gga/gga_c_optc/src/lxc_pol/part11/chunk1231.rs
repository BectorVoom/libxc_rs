//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1231/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1231<F: Float>(t43503: F, t43508: F, t44329: F, t52446: F, t52452: F, t52591: F, t52593: F, t52596: F, t52601: F, t52687: F, t52689: F, t58435: F, t5458: F, t5469: F, t33724: F, t33730: F, t43414: F, t44193: F, t44198: F, t58348: F, t58352: F, t58356: F, t58360: F, t58363: F, t58367: F) -> (F, F, F, F) {
    let t58448 = -0.19388333333333333333e1 * t58435 + 0.14595555555555555556e-2 * t52591 - 0.6568e-2 * t52593 + 0.19704e-1 * t52596 + 0.3284e-2 * t52601 + 0.5170222222222222222e1 * t52446 - 0.15510666666666666667e2 * t52452 - 0.51702222222222222221e1 * t43503 + 0.10340444444444444444e2 * t43508 - 0.821e-2 * t44329 + 0.3284e-2 * t52687 - 0.19704e-1 * t52689;
    let t58464 = t5458 * t5458;
    let t58470 = t5469 * t5469;
    let t58487 = -0.17481481481481481482e3 * t44193 + 0.10488888888888888889e4 * t44198 - 0.12922962962962962963e4 * t43414 + 0.30153580246913580247e4 * t33724 + 0.93234567901234567903e3 * t33730 + 0.58153333333333333332e4 * t58348 + 0.94399999999999999998e3 * t58352 - 0.20977777777777777778e3 * t58356 - 2832.0 * t58360 + 0.62933333333333333332e3 * t58363 - 0.10488888888888888889e3 * t58367;
    (t58448, t58464, t58470, t58487)
}
