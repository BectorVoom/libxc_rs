//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta640 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2050;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta640(t25580: f64, t3053: f64, t23529: f64, t4571: f64, t13961: f64, t6755: f64, t14202: f64, t6765: f64, t13950: f64, t23422: f64, t4603: f64, t14159: f64, t6717: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t88305, t88307, t88320, t88321, t88324, t88335, t88336) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2050(t25580, t3053, t23529, t4571, t13961, t6755, t14202, t6765, t13950, t23422, t4603, t14159, t6717);
    (t88305, t88307, t88320, t88321, t88324, t88335, t88336)
}
