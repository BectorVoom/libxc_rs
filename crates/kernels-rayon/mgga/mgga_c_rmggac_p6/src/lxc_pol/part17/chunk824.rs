//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 824/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk824(t40564: f64, t2320: f64, t35151: f64, t2604: f64, t8997: f64, t2367: f64, t4616: f64, t1679: f64, t7900: f64, t36662: f64, t8417: f64, t1986: f64, t305: f64, t495: f64, t552: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t40565 = 0.24829349937757072982e-4_f64 * t40564;
    let t40566 = t35151 * t2320;
    let t40567 = 0.24829349937757072982e-4_f64 * t40566;
    let t40578 = t2604 * t8997;
    let t40579 = 0.79828278012425390426e-1_f64 * t40578;
    let t40596 = t4616 * t2367;
    let t40623 = t1679 * t7900;
    let t40654 = t36662 * t8417;
    let t40655 = 0.39726959900411316772e-4_f64 * t40654;
    let t40658 = t1986 * t305 * t552 * t495;
    (t40565, t40567, t40579, t40596, t40623, t40655, t40658)
}
