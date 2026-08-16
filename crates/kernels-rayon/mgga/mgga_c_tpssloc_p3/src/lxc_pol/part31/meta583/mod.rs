//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta583 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1823;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1824;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta583(t22704: f64, t5336: f64, t80798: f64, t22724: f64, t26436: f64, t81066: f64, t26423: f64, t81159: f64, t215: f64, t22839: f64, t562: f64, t80854: f64, t81080: f64, t26462: f64, t6914: f64, t22705: f64, t26414: f64, t81228: f64, t26415: f64, t26418: f64, t7736: f64, t81064: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t90898, t90900, t90903, t90912, t90914, t90915) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1823(t22704, t5336, t80798, t22724, t26436, t81066, t26423, t81159, t215, t22839, t562, t80854);
        let (t90925, t90956, t90961, t90963, t90970, t90980) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1824(t81080, t26462, t6914, t22705, t26414, t81228, t26415, t81159, t26418, t7736, t80854, t81064);
    (t90898, t90900, t90903, t90912, t90914, t90915, t90925, t90956, t90961, t90963, t90970, t90980)
}
