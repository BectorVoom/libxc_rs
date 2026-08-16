//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1220/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1220(t10704: f64, t1850: f64, t10636: f64, t5227: f64, t1841: f64, t3487: f64, t7275: f64, t734: f64, t10826: f64, t2536: f64, t1944: f64, t3444: f64) -> (f64, f64, f64, f64, f64) {
    let t32622 = t1850 * t10704;
    let t32623 = 0.85450291446024714264e-3_f64 * t32622;
    let t32625 = 0.17090058289204942853e-2_f64 * t5227 * t10636;
    let t32629 = 0.17090058289204942853e-2_f64 * t1841 * t7275 * t3487 * t734;
    let t32633 = 0.17090058289204942853e-2_f64 * t1841 * t2536 * t10826 * t734;
    let t32634 = t1944 * t3444;
    (t32623, t32625, t32629, t32633, t32634)
}
