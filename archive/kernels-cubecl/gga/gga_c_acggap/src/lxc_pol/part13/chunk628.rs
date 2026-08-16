//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 628/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk628<F: Float>(t4797: F, t4799: F, t4808: F, t4816: F, t3107: F, t4802: F, t4804: F, t4812: F, t4814: F, t4820: F, t4824: F, t3110: F, t3112: F, t3118: F, t3122: F, t3128: F, t3130: F, t3142: F, t3144: F, t3146: F, t3161: F, t4834: F) -> (F, F) {
    let t4856 = t4797 / F::cast_from(6.0_f64);
    let t4857 = F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t4799;
    let t4860 = t4808 / F::cast_from(12.0_f64);
    let t4863 = F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t4816;
    let t4865 = t4856 + t4857 - t4802 / F::cast_from(4.0_f64) + t4804 / F::cast_from(6.0_f64) - t4860 - t4812 / F::cast_from(12.0_f64) - F::cast_from(7.0_f64) / F::cast_from(9.0_f64) * t4814 - t4863 + t4820 + t4824 / F::cast_from(2.0_f64) + t3107;
    let t4874 = -t3110 + t3112 / F::cast_from(3.0_f64) + t3118 / F::cast_from(12.0_f64) - t3122 / F::cast_from(24.0_f64) - t3128 / F::cast_from(6.0_f64) - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t3130 - t3142 - F::cast_from(14.0_f64) / F::cast_from(9.0_f64) * t3144 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t4834 + t3146 / F::cast_from(3.0_f64) + t3161;
    (t4865, t4874)
}
