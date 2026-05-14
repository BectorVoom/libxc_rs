//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1012/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1012<F: Float>(t18210: F, t8212: F, t7978: F, t8225: F, t7974: F, t8218: F, t18256: F, t2256: F, t8209: F, t4409: F, t8217: F, t2260: F, t27362: F, t27567: F, t27583: F, t27592: F, t27602: F, t27604: F, t28384: F, t28395: F, t28407: F, t28738: F, t28749: F, t28755: F, t28760: F, t28767: F, t28772: F, t7968: F) -> (F, F, F, F, F) {
    let t28778 = t18210 * t8212;
    let t28779 = t7978 * t28778;
    let t28781 = t18210 * t8225;
    let t28782 = t7978 * t28781;
    let t28784 = t8218 * t7974;
    let t28788 = t18256 * t2256;
    let t28791 = t8209 * t7974;
    let t28793 = t4409 * t8217;
    let t28797 = -0.46377350260416666667e-4 * t7968 * t28738 + 0.11584201388888888889e-3 * t27583 * t28749 + 0.11584201388888888889e-3 * t27583 * t28755 + 0.23168402777777777778e-3 * t27583 * t28760 + 0.15459116753472222222e-4 * t27567 * t28755 - 0.15445601851851851852e-3 * t27583 * t28767 + 0.77382407407407407407e-3 * t27362 + 0.46377350260416666667e-4 * t7968 * t28772 - 0.3861400462962962963e-4 * t27592 + 0.11584201388888888889e-3 * t27602 + 0.11584201388888888889e-3 * t27604 + 0.11584201388888888889e-3 * t28779 + 0.11584201388888888889e-3 * t28782 + 0.30891203703703703704e-3 * t28784 - 0.38691203703703703703e-3 * t28384 + 0.77382407407407407407e-3 * t28395 - 0.34752604166666666667e-3 * t28788 * t2260 - 0.11584201388888888889e-3 * t28791 + 0.92673611111111111112e-3 * t28793 * t2260 + 0.46429444444444444443e-2 * t28407;
    (t28778, t28781, t28788, t28793, t28797)
}
