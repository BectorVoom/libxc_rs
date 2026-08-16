//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta630 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2042;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta630(t1512: f64, t81824: f64, t23041: f64, t4236: f64, t23040: f64, t4166: f64, t831: f64, t4191: f64, t81749: f64, t4240: f64, t23069: f64, t4159: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t87248, t87256, t87261, t87263, t87271, t87273, t87291) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2042(t1512, t81824, t23041, t4236, t23040, t4166, t831, t4191, t81749, t4240, t23069, t4159);
    (t87248, t87256, t87261, t87263, t87271, t87273, t87291)
}
