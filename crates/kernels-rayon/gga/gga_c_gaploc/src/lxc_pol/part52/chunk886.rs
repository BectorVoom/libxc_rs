//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 886/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk886(t45830: f64, t2676: f64, t36612: f64, t13617: f64, t15751: f64, t11724: f64, t1445: f64, t2530: f64, t813: f64, t13601: f64, t4614: f64, t13616: f64, t5748: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t45831 = 0.57514388930881124514e0_f64 * t45830;
    let t45837 = 0.11916829983950142223e0_f64 * t36612 * t2676;
    let t45848 = 0.27606906686822939767e2_f64 * t15751 * t13617;
    let t45856 = 0.92023022289409799224e1_f64 * t813 * t1445 * t11724 * t2530;
    let t45863 = 0.12269736305254639897e2_f64 * t813 * t4614 * t13601;
    let t45869 = 0.36809208915763919689e2_f64 * t5748 * t4614 * t13616;
    (t45831, t45837, t45848, t45856, t45863, t45869)
}
