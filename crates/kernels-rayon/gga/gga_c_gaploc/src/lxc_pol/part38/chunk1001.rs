//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 1001/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk1001(t13722: f64, t45134: f64, t45148: f64, t45151: f64, t45164: f64, t45971: f64, t45973: f64, t45974: f64, t45978: f64, t45986: f64, t45992: f64, t46000: f64, t46006: f64, t46011: f64, t46023: f64, t46025: f64, t46828: f64, t46830: f64, t46835: f64, t617: f64) -> f64 {
    let t46842 = t13722 * t617 + t45134 + t45148 - t45151 - t45164 + t45971 + t45973 - t45974 - t45978 + t45986 + t45992 + t46000 + t46006 - t46011 + t46023 + t46025 - t46828 - t46830 + t46835;
    t46842
}
