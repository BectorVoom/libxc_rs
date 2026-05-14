//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1105/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1105<F: Float>(t20657: F, t5329: F, t11068: F, t6758: F, t1251: F, t3500: F, t6766: F, t15555: F, t15576: F, t20632: F, t20635: F, t20639: F, t20642: F, t20645: F, t20649: F, t20654: F, t3490: F, t3514: F, t6767: F) -> (F,) {
    let t20658 = t5329 * t20657;
    let t20661 = t11068 * t6758;
    let t20662 = t1251 * t20661;
    let t20666 = t3500 * t6766;
    let t20667 = t1251 * t20666;
    let t20669 = t15555 / 432.0 + t3514 * t20632 / 96.0 - t3514 * t20635 / 72.0 - t3514 * t20639 / 576.0 - t3514 * t20642 / 288.0 + t3514 * t20645 / 432.0 + t15576 + t1251 * t20649 / 576.0 - t1251 * t20654 / 32.0 + t1251 * t20658 / 48.0 + t20662 / 1296.0 + t3490 * t6767 / 108.0 - t20667 / 864.0;
    (t20669,)
}
