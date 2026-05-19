//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1164/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1164<F: Float>(t38003: F, t1055: F, t7918: F, t24063: F, t24064: F, t3332: F, t37988: F, t37998: F, t38002: F, t39937: F, t39940: F, t39942: F, t39943: F, t39945: F, t39947: F) -> F {
    let t39950 = F::cast_from(0.32524801797942610062e-3_f64) * t38003;
    let t39951 = t7918 * t1055;
    let t39954 = t24063 * t3332 * t24064;
    let t39956 = F::cast_from(0.5239643197851989015e-1_f64) * t39937 - t39940 + t39942 + t39943 - F::cast_from(0.13869154784086829701e1_f64) * t37988 - F::cast_from(0.43341108700271342816e-1_f64) * t39945 + F::cast_from(0.2600466522016280569e0_f64) * t39947 + F::cast_from(0.97574405393827830186e-2_f64) * t37998 - t38002 + t39950 + F::cast_from(0.43341108700271342816e-1_f64) * t39951 + F::cast_from(0.13099107994629972538e-1_f64) * t39954;
    t39956
}
