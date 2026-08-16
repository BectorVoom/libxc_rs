//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1049/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1049<F: Float>(t18463: F, t18532: F, t18584: F, t18620: F, t18663: F, t18925: F, t18957: F, t18984: F, t153: F, t156: F, t18054: F, t18367: F, t18369: F, t18372: F, t18375: F, t18377: F, t18379: F, t18413: F, t18415: F, t18416: F, t18419: F, t18420: F, t242: F) -> (F, F) {
    let t18987 = t18463 + t18532 + t18584 + t18620 + t18663 + t18925 + t18957 + t18984;
    let t18991 = F::cast_from(0.10051538464260528225e1_f64) * t18367 + F::cast_from(0.10051538464260528225e1_f64) * t18369 + t18372 - F::cast_from(0.83762820535504401876e-1_f64) * t18054 * t242 - F::cast_from(0.33505128214201760751e0_f64) * t18375 - F::cast_from(0.50257692321302641126e0_f64) * t18377 - F::cast_from(0.33505128214201760751e0_f64) * t18379 - t18413 + t18415 - F::cast_from(0.10051538464260528225e1_f64) * t18416 - t18419 + F::cast_from(0.2010307692852105645e1_f64) * t18420 + F::cast_from(0.42708890021612718669e0_f64) * t153 * t156 * t18987;
    (t18987, t18991)
}
