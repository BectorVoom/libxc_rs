//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta215 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1012;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta215(t2792: f64, t5695: f64, t1547: f64, t2798: f64, t2802: f64, t4335: f64, t5679: f64, t5683: f64, t5687: f64, t894: f64, t2815: f64, t901: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t5697, t5698, t5699, t5705, t5706, t5712, t5714) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1012(t2792, t5695, t1547, t2798, t2802, t4335, t5679, t5683, t5687, t894, t2815, t901);
    (t5697, t5698, t5699, t5705, t5706, t5712, t5714)
}
