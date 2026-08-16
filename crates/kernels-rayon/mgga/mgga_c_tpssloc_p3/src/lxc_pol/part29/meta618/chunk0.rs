//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2060/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2060(t3032: f64, t3508: f64, t24785: f64, t24826: f64, t7368: f64, t85660: f64, t24574: f64, t24781: f64, t24789: f64, t85639: f64, t11553: f64, t2121: f64, t2148: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t85972 = t3032 * t3508;
    let t85984 = t24826 * t24785;
    let t85986 = t85660 * t7368;
    let t85988 = t24574 * t24781;
    let t85996 = t85639 * t24789;
    let t86000 = 0.30461741978670859935e-2_f64 * t2121 * t11553 * t2148;
    (t85972, t85984, t85986, t85988, t85996, t86000)
}
