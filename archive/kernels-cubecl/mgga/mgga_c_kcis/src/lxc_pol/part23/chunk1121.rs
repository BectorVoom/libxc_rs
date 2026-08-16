//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1121/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1121<F: Float>(t18256: F, t2256: F, t7974: F, t8209: F, t4409: F, t8217: F, t2260: F, t27362: F, t27567: F, t27583: F, t27592: F, t27602: F, t27604: F, t28384: F, t28395: F, t28407: F, t28738: F, t28749: F, t28755: F, t28760: F, t28767: F, t28772: F, t28779: F, t28782: F, t28784: F, t7968: F) -> (F, F, F) {
    let t28788 = t18256 * t2256;
    let t28791 = t8209 * t7974;
    let t28793 = t4409 * t8217;
    let t28797 = -F::cast_from(0.46377350260416666667e-4_f64) * t7968 * t28738 + F::cast_from(0.11584201388888888889e-3_f64) * t27583 * t28749 + F::cast_from(0.11584201388888888889e-3_f64) * t27583 * t28755 + F::cast_from(0.23168402777777777778e-3_f64) * t27583 * t28760 + F::cast_from(0.15459116753472222222e-4_f64) * t27567 * t28755 - F::cast_from(0.15445601851851851852e-3_f64) * t27583 * t28767 + F::cast_from(0.77382407407407407407e-3_f64) * t27362 + F::cast_from(0.46377350260416666667e-4_f64) * t7968 * t28772 - F::cast_from(0.3861400462962962963e-4_f64) * t27592 + F::cast_from(0.11584201388888888889e-3_f64) * t27602 + F::cast_from(0.11584201388888888889e-3_f64) * t27604 + F::cast_from(0.11584201388888888889e-3_f64) * t28779 + F::cast_from(0.11584201388888888889e-3_f64) * t28782 + F::cast_from(0.30891203703703703704e-3_f64) * t28784 - F::cast_from(0.38691203703703703703e-3_f64) * t28384 + F::cast_from(0.77382407407407407407e-3_f64) * t28395 - F::cast_from(0.34752604166666666667e-3_f64) * t28788 * t2260 - F::cast_from(0.11584201388888888889e-3_f64) * t28791 + F::cast_from(0.92673611111111111112e-3_f64) * t28793 * t2260 + F::cast_from(0.46429444444444444443e-2_f64) * t28407;
    (t28788, t28793, t28797)
}
