//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 1011/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk1011<F: Float>(t19020: F, t2771: F, t17744: F, t4206: F, t10595: F, t14953: F, t14955: F, t14957: F, t14999: F, t15011: F, t15014: F, t15015: F, t15025: F, t15028: F, t19640: F, t19643: F, t19646: F, t19649: F, t19651: F, t19653: F, t19656: F, t19659: F, t3139: F, t462: F) -> F {
    let t19662 = t2771 * t19020;
    let t19665 = t4206 * t17744;
    let t19668 = -t14953 - t14955 + t14957 - t14999 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t15011 + t15014 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t15015 - F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t15025 - t15028 - F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t462 * t19640 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t3139 * t19643 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t462 * t19646 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t19649 + t19651 / F::cast_from(3.0_f64) - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t19653 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t10595 + t462 * t19656 / F::cast_from(3.0_f64) - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t462 * t19659 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t462 * t19662 - F::cast_from(2.0_f64) * t462 * t19665;
    t19668
}
