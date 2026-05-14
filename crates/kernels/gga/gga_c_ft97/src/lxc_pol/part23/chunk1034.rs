//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1034/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1034<F: Float>(t1200: F, t817: F, t285: F, t1701: F, t5295: F, t6027: F, t1208: F, t2035: F, t6979: F, t5284: F, t27494: F, t1196: F, t1201: F, t1472: F, t14729: F, t14742: F, t19107: F, t28558: F, t28603: F, t292: F, t30622: F, t30677: F, t30696: F, t30700: F, t4094: F, t4099: F, t4104: F) -> (F, F, F, F, F, F) {
    let t31462 = t1200 * t817;
    let t31465 = t285 * t817;
    let t31473 = t1701 * t6027 * t5295;
    let t31477 = t2035 * t6979 * t1208;
    let t31481 = t1701 * t6027 * t5284;
    let t31485 = t1701 * t27494 * t1208;
    let t31489 = t1701 * t27494 * t1196;
    let t31498 = 0.80559205902449556552e-1 * t28603 * t30622 - 0.80559205902449556552e-1 * t28558 * t30622 - 0.18611243628760286395e2 * t1201 * t30700 + 0.56502890877451119026e-1 * t31462 * t30677 - 0.28251445438725559513e-1 * t31465 * t30677 - 0.76518236253115177207e1 * t292 * t30696 + 0.93056218143801431977e1 * t292 * t30700 - 0.22653425206514361674e0 * t4099 * t31473 - 0.21895580739717983995e1 * t19107 * t31477 - 0.45306850413028723348e0 * t14729 * t31481 - 0.24163653553615319118e1 * t1472 * t31485 + 0.48327307107230638237e1 * t4104 * t31489 + 0.45306850413028723348e0 * t14742 * t31481 + 0.24163653553615319118e1 * t4099 * t31485 - 0.48327307107230638237e1 * t4094 * t31489;
    (t31462, t31465, t31473, t31477, t31489, t31498)
}
