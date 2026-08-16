//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta423 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1629;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1630;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta423(t19164: f64, t19207: f64, t1241: f64, t1235: f64, t6150: f64, t1760: f64, t5088: f64, t3598: f64, t1251: f64, t6267: f64, t6243: f64, t11606: f64, t1238: f64, t15820: f64, t1761: f64, t18287: f64, t19121: f64, t3487: f64, t3593: f64, t4945: f64, t498: f64, t5055: f64, t5060: f64, t6268: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t19208, t19209, t19211, t19213, t19214, t19219, t19220, t19225, t19226) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1629(t19164, t19207, t1241, t1235, t6150, t1760, t5088, t3598, t1251, t6267, t6243, t11606);
        let t19231 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1630(t1238, t15820, t1761, t18287, t19121, t19209, t19211, t19214, t19220, t19226, t3487, t3593, t4945, t498, t5055, t5060, t6268);
    (t19208, t19209, t19211, t19213, t19214, t19219, t19220, t19225, t19226, t19231)
}
