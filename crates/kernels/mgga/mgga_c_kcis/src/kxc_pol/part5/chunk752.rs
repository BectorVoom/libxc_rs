//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 752/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk752<F: Float>(t1386: F, t5737: F, t1364: F, t3729: F, t3731: F, t4143: F, t507: F, t5418: F, t5623: F, t5630: F, t5635: F, t5639: F, t5646: F, t5651: F, t5657: F, t5665: F, t5669: F, t5674: F, t5679: F, t5681: F, t5684: F, t5686: F) -> (F, F) {
    let t5738 = t5737 * t1386;
    let t5741 = -F::cast_from(0.24872916666666666666e-2_f64) * t5418 - F::cast_from(0.24872916666666666666e-2_f64) * t5630 + F::cast_from(0.66327777777777777776e-2_f64) * t5635 + F::cast_from(0.11054629629629629629e-2_f64) * t5639 - F::cast_from(0.16581944444444444444e-2_f64) * t3729 + F::cast_from(0.11054629629629629629e-2_f64) * t3731 - F::cast_from(0.16581944444444444444e-2_f64) * t5646 + F::cast_from(0.11054629629629629629e-2_f64) * t5651 - F::cast_from(0.33163888888888888888e-2_f64) * t5657 + F::cast_from(0.27636574074074074073e-2_f64) * t5665 - F::cast_from(0.16581944444444444444e-2_f64) * t5669 - F::cast_from(0.16581944444444444444e-2_f64) * t5674 + F::cast_from(0.49745833333333333332e-2_f64) * t5679 + F::cast_from(0.11054629629629629629e-2_f64) * t5681 + F::cast_from(0.11054629629629629629e-2_f64) * t4143 - F::cast_from(0.44218518518518518517e-2_f64) * t5684 + F::cast_from(0.16581944444444444444e-2_f64) * t5686 + t5623 * t507 - F::new(0.66725e-1) * t1364 * t5738;
    (t5738, t5741)
}
