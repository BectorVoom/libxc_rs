//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta569 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1938;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1939;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta569(t343: f64, t5836: f64, t6734: f64, t5842: f64, t1941: f64, t5904: f64, t1011: f64, t5872: f64, t3131: f64, t23512: f64, t360: f64, t23519: f64, t5866: f64, t68: f64, t6744: f64, t1935: f64, t23419: f64, t23469: f64, t23510: f64, t25639: f64, t25642: f64, t25683: f64, t378: f64, t5885: f64, t5890: f64, t5894: f64, t5900: f64, t5909: f64, t6717: f64, t6742: f64, t6765: f64, t7574: f64, t7578: f64, t7583: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t28557, t28558, t28565, t28566, t28572, t28577, t28578, t28581, t28582) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1938(t343, t5836, t6734, t5842, t1941, t5904, t1011, t5872, t3131, t23512, t360, t23519);
        let (t28586, t28587, t28592) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1939(t360, t5866, t68, t6744, t1935, t23419, t23469, t23510, t25639, t25642, t25683, t28558, t28566, t28572, t28578, t28582, t378, t5885, t5890, t5894, t5900, t5909, t6717, t6742, t6765, t7574, t7578, t7583);
    (t28557, t28558, t28565, t28566, t28572, t28577, t28578, t28581, t28582, t28586, t28587, t28592)
}
