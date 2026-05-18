//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1304/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1304<F: Float>(t27651: F, t8218: F, t98597: F, t98603: F, t18210: F, t28834: F, t7978: F, t1598: F, t251: F, t54605: F, t12581: F, t2260: F, t7971: F, t7986: F, t8213: F, t8217: F, t95021: F, t95123: F, t95125: F, t98609: F, t98620: F, t99013: F, t99219: F) -> (F, F, F) {
    let t99476 = t8218 * t27651;
    let t99478 = F::new(0.23214722222222222222e-2) * t98597;
    let t99480 = F::new(0.23214722222222222222e-2) * t98603;
    let t99494 = F::new(0.23168402777777777778e-3) * t7978 * t18210 * t28834;
    let t99497 = t54605 * t251 * t1598;
    let t99501 = -t99480 + F::new(0.34752604166666666667e-3) * t95021 * t8213 + F::new(0.69505208333333333334e-3) * t99013 * t7986 + F::new(0.92673611111111111112e-3) * t12581 * t8217 * t2260 - F::new(0.23168402777777777778e-3) * t95123 - F::new(0.18534722222222222222e-2) * t99219 * t7971 - F::new(0.34822083333333333332e-2) * t98609 + t99494 - F::new(0.46377350260416666666e-4) * t95125 + F::new(0.92754700520833333334e-4) * t99497 * t7971 + F::new(0.23214722222222222222e-2) * t98620;
    (t99476, t99478, t99501)
}
