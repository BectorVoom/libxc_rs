//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1118/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1118<F: Float>(t1610: F, t2109: F, t27614: F, t6176: F, t4425: F, t8221: F, t7978: F, t27335: F, t27337: F, t27556: F, t27564: F, t27567: F, t27595: F, t28329: F, t28333: F, t28340: F, t28358: F, t28362: F, t28701: F, t28708: F, t28714: F, t28721: F, t28727: F, t7968: F, t7971: F, t7981: F, t7986: F, t8213: F) -> (F, F, F, F, F) {
    let t28736 = t2109 * t1610;
    let t28737 = t27614 * t28736;
    let t28738 = t6176 * t28737;
    let t28741 = t4425 * t8221;
    let t28742 = t7978 * t28741;
    let t28744 = F::cast_from(0.15459116753472222222e-4_f64) * t27567 * t28701 + F::cast_from(0.11607361111111111111e-2_f64) * t28329 - F::cast_from(0.23214722222222222222e-2_f64) * t28333 - F::cast_from(0.13913205078125e-3_f64) * t7968 * t28708 - F::cast_from(0.92835860883789062501e-5_f64) * t27595 * t28708 - F::cast_from(0.11584201388888888889e-3_f64) * t28714 * t7981 - F::cast_from(0.11607361111111111111e-2_f64) * t28340 + F::cast_from(0.34752604166666666667e-3_f64) * t28714 * t7971 + F::cast_from(0.46377350260416666667e-4_f64) * t28721 * t7971 + F::cast_from(0.46377350260416666667e-4_f64) * t27556 * t8213 + F::cast_from(0.15459116753472222222e-4_f64) * t27564 - F::cast_from(0.92673611111111111112e-3_f64) * t28727 * t7986 - F::cast_from(0.69505208333333333334e-3_f64) * t7978 * t28708 - F::cast_from(0.30952962962962962962e-2_f64) * t28358 - F::cast_from(0.11607361111111111111e-2_f64) * t28362 - F::cast_from(0.11607361111111111111e-2_f64) * t27335 + F::cast_from(0.77382407407407407407e-3_f64) * t27337 - F::cast_from(0.34752604166666666667e-3_f64) * t7978 * t28738 - F::cast_from(0.3861400462962962963e-4_f64) * t28742;
    (t28736, t28737, t28738, t28741, t28744)
}
