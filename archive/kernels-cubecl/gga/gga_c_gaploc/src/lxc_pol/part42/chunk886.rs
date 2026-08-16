//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 886/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk886<F: Float>(t45830: F, t2676: F, t36612: F, t13617: F, t15751: F, t11724: F, t1445: F, t2530: F, t813: F, t13601: F, t4614: F, t13616: F, t5748: F) -> (F, F, F, F, F, F) {
    let t45831 = F::cast_from(0.57514388930881124514e0_f64) * t45830;
    let t45837 = F::cast_from(0.11916829983950142223e0_f64) * t36612 * t2676;
    let t45848 = F::cast_from(0.27606906686822939767e2_f64) * t15751 * t13617;
    let t45856 = F::cast_from(0.92023022289409799224e1_f64) * t813 * t1445 * t11724 * t2530;
    let t45863 = F::cast_from(0.12269736305254639897e2_f64) * t813 * t4614 * t13601;
    let t45869 = F::cast_from(0.36809208915763919689e2_f64) * t5748 * t4614 * t13616;
    (t45831, t45837, t45848, t45856, t45863, t45869)
}
