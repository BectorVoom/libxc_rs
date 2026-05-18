//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1111/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1111<F: Float>(t3616: F, t7773: F, t5329: F, t15573: F, t7795: F, t7788: F, t1268: F, t3530: F, t3532: F, t26751: F, t26755: F, t26764: F, t26774: F, t26977: F, t26993: F, t26999: F, t27003: F, t27007: F, t27010: F, t27014: F, t7772: F, t7791: F) -> (F, F, F, F, F, F, F, F) {
    let t27019 = t7773 * t3616;
    let t27020 = t5329 * t27019;
    let t27023 = t15573 * t7795;
    let t27024 = t7788 * t27023;
    let t27028 = t3530 * t1268;
    let t27029 = t27028 * t3532;
    let t27030 = t5329 * t27029;
    let t27037 = F::new(0.23168402777777777778e-3) * t7788 * t26993 - F::new(0.69505208333333333334e-3) * t7788 * t26999 - F::new(0.15445601851851851852e-3) * t7788 * t27003 - F::new(0.7722800925925925926e-4) * t27007 - F::new(0.11584201388888888889e-3) * t7788 * t27010 - F::new(0.23168402777777777778e-3) * t27014 * t7791 - F::new(0.92754700520833333334e-4) * t7772 * t26999 + F::new(0.34752604166666666667e-3) * t7788 * t27020 + F::new(0.23168402777777777778e-3) * t27024 - F::new(0.69505208333333333334e-3) * t7788 * t26977 - F::new(0.69505208333333333334e-3) * t7788 * t27030 + F::new(0.15476481481481481481e-2) * t26751 + F::new(0.23214722222222222222e-2) * t26755 - F::new(0.23214722222222222222e-2) * t26764 + F::new(0.34822083333333333332e-2) * t26774;
    (t27019, t27020, t27023, t27024, t27028, t27029, t27030, t27037)
}
