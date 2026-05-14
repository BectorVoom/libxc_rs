//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 741/964 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk741<F: Float>(t40558: F, t34884: F, t8831: F, t8836: F, t8843: F, t2320: F, t35151: F, t2604: F, t8997: F, t2367: F, t4616: F, t1679: F, t7900: F, t36662: F, t8417: F, t1986: F, t305: F, t495: F, t552: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t40559 = 0.24829349937757072982e-4 * t40558;
    let t40560 = t34884 * t8831;
    let t40561 = 0.74488049813271218946e-4 * t40560;
    let t40562 = t34884 * t8836;
    let t40563 = 0.74488049813271218946e-4 * t40562;
    let t40564 = t34884 * t8843;
    let t40565 = 0.24829349937757072982e-4 * t40564;
    let t40566 = t35151 * t2320;
    let t40567 = 0.24829349937757072982e-4 * t40566;
    let t40578 = t2604 * t8997;
    let t40579 = 0.79828278012425390426e-1 * t40578;
    let t40596 = t4616 * t2367;
    let t40623 = t1679 * t7900;
    let t40654 = t36662 * t8417;
    let t40655 = 0.39726959900411316772e-4 * t40654;
    let t40658 = t1986 * t305 * t552 * t495;
    (t40559, t40561, t40563, t40565, t40567, t40579, t40596, t40623, t40655, t40658)
}
