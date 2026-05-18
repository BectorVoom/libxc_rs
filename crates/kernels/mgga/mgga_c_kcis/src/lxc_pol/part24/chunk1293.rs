//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1293/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1293<F: Float>(t1008: F, t27772: F, t2811: F, t6533: F, t100986: F, t26685: F, t1003: F, t100447: F, t100451: F, t101066: F, t101111: F, t19396: F, t27832: F, t27904: F, t44657: F, t4939: F, t7703: F, t8038: F, t93463: F, t95640: F, t95877: F, t95892: F) -> (F, F) {
    let t101231 = t27772 * t2811 * t6533 * t1008;
    let t101235 = t26685 * t100986;
    let t101237 = -t95877 + F::new(0.30891203703703703704e-3) * t7703 * t101066 + F::new(0.18534722222222222223e-2) * t7703 * t4939 * t93463 * t19396 - F::new(0.46336805555555555556e-3) * t95640 * t8038 - F::new(0.92673611111111111112e-3) * t27832 * t27904 + F::new(0.41703125000000000001e-2) * t7703 * t44657 * t101111 * t1003 - F::new(0.1492375e-1) * t100447 + F::new(0.13901041666666666667e-2) * t7703 * t101231 + F::new(0.33163888888888888888e-2) * t100451 - t95892 + F::new(0.20612155671296296296e-4) * t101235;
    (t101231, t101237)
}
