//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1295/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1295<F: Float>(t29895: F, t30517: F, t29900: F, t30524: F, t30527: F, t110075: F, t30507: F, t110082: F, t110314: F, t111101: F, t111104: F, t111109: F, t111111: F, t19517: F, t30063: F, t30175: F, t5480: F, t662: F, t8137: F, t8180: F, t96715: F) -> F {
    let t111763 = t29895 * t30517;
    let t111765 = t29900 * t30524;
    let t111767 = t29900 * t30527;
    let t111769 = t110075 * t30507;
    let t111772 = F::cast_from(5.0_f64) / F::cast_from(108.0_f64) * t8137 * t110314 * t5480 * t662 + F::cast_from(5.0_f64) / F::cast_from(18.0_f64) * t30175 * t30063 * t19517 + F::cast_from(3.0_f64) * t110082 * t8180 * t96715 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t111763 - F::cast_from(50.0_f64) / F::cast_from(27.0_f64) * t111765 + F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t111767 + F::cast_from(2.0_f64) * t111769 + F::cast_from(44.0_f64) / F::cast_from(9.0_f64) * t111101 - t111104 + t111109 - t111111;
    t111772
}
