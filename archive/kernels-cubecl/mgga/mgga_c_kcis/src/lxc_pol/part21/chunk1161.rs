//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1161/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1161<F: Float>(t26955: F, t26960: F, t26966: F, t27042: F, t27766: F, t27770: F, t27786: F, t27790: F, t27794: F, t27797: F, t27799: F, t28094: F, t28098: F, t28102: F, t28107: F, t28113: F, t28118: F, t28125: F, t28132: F, t28137: F, t7772: F, t8087: F, t8091: F) -> F {
    let t28140 = F::cast_from(0.34822083333333333332e-2_f64) * t27766 - F::cast_from(0.11607361111111111111e-2_f64) * t27770 + F::cast_from(0.77382407407407407407e-3_f64) * t27786 - F::cast_from(0.11607361111111111111e-2_f64) * t27790 - F::cast_from(0.38691203703703703703e-3_f64) * t27794 - F::cast_from(0.30952962962962962963e-2_f64) * t27797 + F::cast_from(0.77382407407407407407e-3_f64) * t27799 - F::cast_from(0.12367293402777777778e-3_f64) * t27042 * t8087 + F::cast_from(0.15459116753472222222e-4_f64) * t28094 + F::cast_from(0.11584201388888888889e-3_f64) * t26960 * t28098 + F::cast_from(0.11584201388888888889e-3_f64) * t26960 * t28102 + F::cast_from(0.11584201388888888889e-3_f64) * t26960 * t28107 + F::cast_from(0.11584201388888888889e-3_f64) * t26960 * t28113 + F::cast_from(0.23168402777777777778e-3_f64) * t26960 * t28118 + F::cast_from(0.15459116753472222222e-4_f64) * t26955 * t28113 - F::cast_from(0.15445601851851851852e-3_f64) * t26960 * t28125 + F::cast_from(0.30891203703703703704e-3_f64) * t26966 * t8091 - F::cast_from(0.46377350260416666667e-4_f64) * t7772 * t28132 - F::cast_from(0.13913205078125e-3_f64) * t7772 * t28137;
    t28140
}
