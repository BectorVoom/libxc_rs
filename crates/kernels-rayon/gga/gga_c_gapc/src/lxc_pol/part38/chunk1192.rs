//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 1192/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk1192(t11587: f64, t11591: f64, t3060: f64, t28006: f64, t3112: f64, t33498: f64, t8362: f64, t11488: f64, t1688: f64, t21157: f64, t1743: f64, t33958: f64, t34711: f64) -> (f64, f64, f64, f64) {
    let t34772 = t3060 * t11587 * t11591;
    let t34776 = t3112 * t33498 * t8362 * t28006;
    let t34779 = t11488 * t1688 * t21157;
    let t34782 = t1743 * t33958 * t34711;
    (t34772, t34776, t34779, t34782)
}
