//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 750/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk750(t3698: f64, t8785: f64, t8784: f64, t1672: f64, t3142: f64, t1462: f64, t2993: f64, t3120: f64, t1036: f64, t1699: f64, t3144: f64, t8620: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8786 = t3698 * t8785;
    let t8787 = t8784 * t8786;
    let t8788 = t1672 * t3142;
    let t8789 = t1462 * t8788;
    let t8790 = t8787 * t8789;
    let t8792 = t2993 * t3120;
    let t8793 = t1036 * t1699;
    let t8794 = t8792 * t8793;
    let t8796 = t8620 * t3144;
    (t8786, t8787, t8788, t8789, t8790, t8793, t8794, t8796)
}
