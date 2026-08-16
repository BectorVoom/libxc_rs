//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta415 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1586;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1587;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta415(t22792: f64, t22794: f64, t547: f64, t6546: f64, t1329: f64, t3770: f64, t6916: f64, t2230: f64, t6924: f64, t213: f64, t6928: f64, t1998: f64, t236: f64, t3719: f64, t6926: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t22795, t22797) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1586(t22792, t22794, t547, t6546);
        let (t22798, t22799, t22800, t22803, t22804, t22805, t22808, t22809) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1587(t1329, t22797, t3770, t6916, t2230, t6924, t213, t6928, t1998, t236, t3719, t6926);
    (t22795, t22797, t22798, t22799, t22800, t22803, t22804, t22805, t22808, t22809)
}
