//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1111/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1111(t3616: f64, t7773: f64, t5329: f64, t15573: f64, t7795: f64, t7788: f64, t1268: f64, t3530: f64, t3532: f64, t26751: f64, t26755: f64, t26764: f64, t26774: f64, t26977: f64, t26993: f64, t26999: f64, t27003: f64, t27007: f64, t27010: f64, t27014: f64, t7772: f64, t7791: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t27019 = t7773 * t3616;
    let t27020 = t5329 * t27019;
    let t27023 = t15573 * t7795;
    let t27024 = t7788 * t27023;
    let t27028 = t3530 * t1268;
    let t27029 = t27028 * t3532;
    let t27030 = t5329 * t27029;
    let t27037 = 0.23168402777777777778e-3_f64 * t7788 * t26993 - 0.69505208333333333334e-3_f64 * t7788 * t26999 - 0.15445601851851851852e-3_f64 * t7788 * t27003 - 0.7722800925925925926e-4_f64 * t27007 - 0.11584201388888888889e-3_f64 * t7788 * t27010 - 0.23168402777777777778e-3_f64 * t27014 * t7791 - 0.92754700520833333334e-4_f64 * t7772 * t26999 + 0.34752604166666666667e-3_f64 * t7788 * t27020 + 0.23168402777777777778e-3_f64 * t27024 - 0.69505208333333333334e-3_f64 * t7788 * t26977 - 0.69505208333333333334e-3_f64 * t7788 * t27030 + 0.15476481481481481481e-2_f64 * t26751 + 0.23214722222222222222e-2_f64 * t26755 - 0.23214722222222222222e-2_f64 * t26764 + 0.34822083333333333332e-2_f64 * t26774;
    (t27019, t27020, t27023, t27024, t27028, t27029, t27030, t27037)
}
