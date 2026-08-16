//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta612 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1925;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1926;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta612(t22863: f64, t7737: f64, t26448: f64, t90497: f64, t215: f64, t6916: f64, t225: f64, t3787: f64, t562: f64, t16313: f64, t22751: f64, t26385: f64, t16068: f64, t1992: f64, t6976: f64, t26395: f64, t3719: f64, t6637: f64, t6888: f64, t16307: f64, t90915: f64, t1307: f64, t26331: f64, t26446: f64, t90818: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t91000, t91002, t91004, t91005, t91008, t91010) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1925(t22863, t7737, t26448, t90497, t215, t6916, t225, t3787, t562, t16313, t22751, t26385);
        let (t91014, t91025, t91036, t91048) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1926(t16068, t1992, t6976, t26395, t3719, t6637, t6888, t16307, t90915, t91004, t1307, t26331, t26446, t90818);
    (t91000, t91002, t91005, t91008, t91010, t91014, t91025, t91036, t91048)
}
