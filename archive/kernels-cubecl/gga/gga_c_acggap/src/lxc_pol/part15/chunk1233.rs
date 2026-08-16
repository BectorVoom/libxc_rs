//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1233/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1233<F: Float>(t30868: F, t30872: F, t30880: F, t32619: F, t35025: F, t35028: F, t35030: F, t39615: F, t39617: F, t39620: F, t39623: F, t39626: F, t39629: F, t39632: F, t39640: F, t39643: F, t39647: F, t39649: F) -> F {
    let t41736 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t39615 - t39617 / F::cast_from(48.0_f64) - t39620 / F::cast_from(32.0_f64) + t39623 / F::cast_from(48.0_f64) + F::cast_from(0.305625e-1_f64) * t39626 - t39629 / F::cast_from(2.0_f64) + t39632 / F::cast_from(24.0_f64) - t35025 + F::cast_from(0.45351183609335988442e-1_f64) * t30868 - F::cast_from(0.45351183609335988442e-1_f64) * t30872 + t35028 - t35030 + F::cast_from(0.90035438047946447644e-2_f64) * t30880 + t32619 + F::cast_from(0.42874018118069736972e-3_f64) * t39640 + F::cast_from(0.42874018118069736972e-3_f64) * t39643 + F::cast_from(0.28582678745379824648e-3_f64) * t39647 - F::cast_from(0.17149607247227894789e-1_f64) * t39649;
    t41736
}
