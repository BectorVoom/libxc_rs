//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1304/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1304(t27651: f64, t8218: f64, t98597: f64, t98603: f64, t18210: f64, t28834: f64, t7978: f64, t1598: f64, t251: f64, t54605: f64, t12581: f64, t2260: f64, t7971: f64, t7986: f64, t8213: f64, t8217: f64, t95021: f64, t95123: f64, t95125: f64, t98609: f64, t98620: f64, t99013: f64, t99219: f64) -> (f64, f64, f64) {
    let t99476 = t8218 * t27651;
    let t99478 = 0.23214722222222222222e-2_f64 * t98597;
    let t99480 = 0.23214722222222222222e-2_f64 * t98603;
    let t99494 = 0.23168402777777777778e-3_f64 * t7978 * t18210 * t28834;
    let t99497 = t54605 * t251 * t1598;
    let t99501 = -t99480 + 0.34752604166666666667e-3_f64 * t95021 * t8213 + 0.69505208333333333334e-3_f64 * t99013 * t7986 + 0.92673611111111111112e-3_f64 * t12581 * t8217 * t2260 - 0.23168402777777777778e-3_f64 * t95123 - 0.18534722222222222222e-2_f64 * t99219 * t7971 - 0.34822083333333333332e-2_f64 * t98609 + t99494 - 0.46377350260416666666e-4_f64 * t95125 + 0.92754700520833333334e-4_f64 * t99497 * t7971 + 0.23214722222222222222e-2_f64 * t98620;
    (t99476, t99478, t99501)
}
