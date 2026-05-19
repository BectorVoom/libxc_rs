//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 444/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk444<F: Float>(t1056: F, t1662: F, t1064: F, t1079: F, t1030: F, t104: F, t1050: F, t1055: F, t1063: F, t1069: F, t1072: F, t1078: F, t111: F, t120: F, t1646: F) -> (F, F, F, F) {
    let t1734 = t1056 * t1662;
    let t1737 = t1064 * t1662;
    let t1742 = t1079 * t1662;
    let t1745 = t1050 + F::cast_from(0.11955719325063177623e-1_f64) * t1030 * t1646 - t1055 - F::new(0.3513e-2) * t104 * t1734 + t1063 + F::new(0.7925e-3) * t111 * t1737 - t1069 - F::cast_from(0.5179538907796306876e-4_f64) * t1072 * t1646 + t1078 + F::new(0.50413125e-5) * t120 * t1742;
    (t1734, t1737, t1742, t1745)
}
