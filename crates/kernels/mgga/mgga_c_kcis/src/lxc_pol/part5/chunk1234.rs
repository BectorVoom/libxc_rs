//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1234/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1234<F: Float>(t1251: F, t20666: F, t15555: F, t15576: F, t20632: F, t20635: F, t20639: F, t20642: F, t20645: F, t20649: F, t20654: F, t20658: F, t20662: F, t3490: F, t3514: F, t6767: F) -> F {
    let t20667 = t1251 * t20666;
    let t20669 = t15555 / F::new(432.0) + t3514 * t20632 / F::new(96.0) - t3514 * t20635 / F::new(72.0) - t3514 * t20639 / F::new(576.0) - t3514 * t20642 / F::new(288.0) + t3514 * t20645 / F::new(432.0) + t15576 + t1251 * t20649 / F::new(576.0) - t1251 * t20654 / F::new(32.0) + t1251 * t20658 / F::new(48.0) + t20662 / F::new(1296.0) + t3490 * t6767 / F::new(108.0) - t20667 / F::new(864.0);
    t20669
}
