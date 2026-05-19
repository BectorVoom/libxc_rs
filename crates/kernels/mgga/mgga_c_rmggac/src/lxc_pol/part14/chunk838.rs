//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 838/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk838<F: Float>(t1550: F, t2060: F, t27146: F, t1356: F, t1364: F, t2024: F, t27124: F, t27177: F, t30900: F, t34688: F, t36280: F, t38643: F, t38645: F, t38648: F, t38653: F, t38658: F, t38663: F, t38676: F, t38678: F, t38680: F, t4985: F, t739: F, t7533: F, t7567: F, t8377: F) -> F {
    let t38685 = t1550 * t2060 * t27146;
    let t38693 = -F::cast_from(0.59590439850616975157e-4_f64) * t38643 + F::cast_from(0.59590439850616975157e-4_f64) * t38645 + t38648 - t34688 + F::cast_from(0.25538759935978703638e-4_f64) * t38653 - F::cast_from(0.25538759935978703638e-4_f64) * t38658 - F::cast_from(0.85129199786595678796e-5_f64) * t38663 + F::cast_from(0.23948483403727617128e0_f64) * t739 * t7567 * t8377 + F::cast_from(0.23948483403727617128e0_f64) * t739 * t2024 * t27146 + F::cast_from(0.11974241701863808564e0_f64) * t739 * t2024 * t27124 - t38676 + F::cast_from(0.2993560425465952141e-1_f64) * t38678 + F::cast_from(0.5987120850931904282e-1_f64) * t38680 - F::cast_from(0.23948483403727617128e0_f64) * t4985 * t7533 + F::cast_from(0.5987120850931904282e-1_f64) * t38685 + F::cast_from(0.47896966807455234256e0_f64) * t1364 * t2024 * t27177 + F::cast_from(0.47896966807455234256e0_f64) * t1356 * t36280 * t30900;
    t38693
}
