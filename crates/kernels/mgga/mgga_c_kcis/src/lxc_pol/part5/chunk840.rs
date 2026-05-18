//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 840/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk840<F: Float>(t413: F, t6835: F, t1260: F, t286: F, t1251: F, t3499: F, t3514: F, t5300: F, t5322: F, t6759: F, t6763: F, t6767: F, t6771: F, t6776: F) -> (F, F, F, F) {
    let t418 = F::new(0.0) < t413;
    let t6837 = piecewise3::<f64>(t418, t6835, -t6835);
    let t6838 = t1260 * t6837;
    let t6839 = t286 * t6838;
    let t6842 = -t3499 + t5300 / F::new(864.0) - t5322 / F::new(288.0) + t1251 * t6759 / F::new(432.0) - t3514 * t6763 / F::new(288.0) - t1251 * t6767 / F::new(288.0) + t1251 * t6771 / F::new(576.0) + t1251 * t6776 / F::new(96.0) - t1251 * t6839 / F::new(192.0);
    (t6837, t6838, t6839, t6842)
}
