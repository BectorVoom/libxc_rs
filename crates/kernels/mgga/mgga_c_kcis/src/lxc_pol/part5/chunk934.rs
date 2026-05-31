//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 934/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk934<F: Float>(t2593: F, t62: F, t2526: F, t7617: F, t153: F, t7627: F, t818: F, t8755: F, t9024: F, t9026: F, t9028: F, t9031: F, t9034: F, t9036: F, t9038: F, t9040: F, t9043: F, t9048: F, t9050: F) -> (F, F, F, F) {
    let t9052 = t2593 * t62;
    let t9053 = t7617 * t2526;
    let t9054 = t9052 * t9053;
    let t9056 = t153 * t7627;
    let t9058 = t8755 * t818;
    let t9060 = F::cast_from(0.3375e1_f64) * t9024 - F::cast_from(0.2428125e1_f64) * t9026 + F::cast_from(0.225e1_f64) * t9028 - F::cast_from(0.485625e0_f64) * t9031 + F::cast_from(0.2428125e1_f64) * t9034 - F::cast_from(0.3375e1_f64) * t9036 - F::cast_from(0.97125e0_f64) * t9038 + F::cast_from(0.485625e0_f64) * t9040 + F::cast_from(0.1125e1_f64) * t9043 - F::cast_from(0.2428125e0_f64) * t9048 - F::cast_from(0.2428125e0_f64) * t9050 + F::cast_from(0.1125e1_f64) * t9054 - F::cast_from(0.45e1_f64) * t9056 + F::cast_from(0.12140625e0_f64) * t9058;
    (t9054, t9056, t9058, t9060)
}
