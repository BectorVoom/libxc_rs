//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1164/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1164(t38003: f64, t1055: f64, t7918: f64, t24063: f64, t24064: f64, t3332: f64, t37988: f64, t37998: f64, t38002: f64, t39937: f64, t39940: f64, t39942: f64, t39943: f64, t39945: f64, t39947: f64) -> f64 {
    let t39950 = 0.32524801797942610062e-3_f64 * t38003;
    let t39951 = t7918 * t1055;
    let t39954 = t24063 * t3332 * t24064;
    let t39956 = 0.5239643197851989015e-1_f64 * t39937 - t39940 + t39942 + t39943 - 0.13869154784086829701e1_f64 * t37988 - 0.43341108700271342816e-1_f64 * t39945 + 0.2600466522016280569e0_f64 * t39947 + 0.97574405393827830186e-2_f64 * t37998 - t38002 + t39950 + 0.43341108700271342816e-1_f64 * t39951 + 0.13099107994629972538e-1_f64 * t39954;
    t39956
}
