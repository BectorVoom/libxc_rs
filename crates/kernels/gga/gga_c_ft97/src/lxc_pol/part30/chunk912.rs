//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 912/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk912<F: Float>(t27501: F, t33404: F, t27633: F, t140869: F, t33366: F, t6804: F, t6762: F, t695: F, t1541: F, t2404: F, t33435: F, t3746: F, t141166: F, t150655: F, t27521: F, t27658: F, t27665: F, t33351: F, t33367: F, t33374: F, t33388: F, t33390: F, t33445: F, t35367: F, t3730: F, t3755: F, t3818: F, t7447: F) -> (F, F, F, F) {
    let t150876 = t33404 * t27501;
    let t150879 = t33404 * t27633;
    let t150883 = t33366 * t140869 * t6804;
    let t150887 = t6762 * t695;
    let t150902 = t33435 * t1541 * t2404 * t3746;
    let t150907 = -0.22705522127871165896e-3 * t27658 * t150876 + 0.15137014751914110597e-3 * t27658 * t150879 + 0.29693535778629056444e-3 * t150883 + 0.38731446812548799881e-3 * t33351 * t3755 - 0.22227677429409423704e-2 * t150887 * t33390 + 0.23254900946437792e-1 * t141166 * t3730 + 0.89080607335887169333e-3 * t33366 * t33367 * t27665 + 0.11854761295685025975e-1 * t33388 * t150655 + 0.5449325310689079815e-2 * t27521 * t35367 * t33374 - 0.78259321553885081522e-2 * t33445 * t150902 - 2.0 * t7447 * t3818;
    (t150876, t150879, t150902, t150907)
}
