//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1041/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1041<F: Float>(t141166: F, t150655: F, t150876: F, t150879: F, t150883: F, t150887: F, t150902: F, t27521: F, t27658: F, t27665: F, t33351: F, t33366: F, t33367: F, t33374: F, t33388: F, t33390: F, t33445: F, t35367: F, t3730: F, t3755: F, t3818: F, t7447: F) -> F {
    let t150907 = -F::cast_from(0.22705522127871165896e-3_f64) * t27658 * t150876 + F::cast_from(0.15137014751914110597e-3_f64) * t27658 * t150879 + F::cast_from(0.29693535778629056444e-3_f64) * t150883 + F::cast_from(0.38731446812548799881e-3_f64) * t33351 * t3755 - F::cast_from(0.22227677429409423704e-2_f64) * t150887 * t33390 + F::cast_from(0.23254900946437792e-1_f64) * t141166 * t3730 + F::cast_from(0.89080607335887169333e-3_f64) * t33366 * t33367 * t27665 + F::cast_from(0.11854761295685025975e-1_f64) * t33388 * t150655 + F::cast_from(0.5449325310689079815e-2_f64) * t27521 * t35367 * t33374 - F::cast_from(0.78259321553885081522e-2_f64) * t33445 * t150902 - F::new(2.0) * t7447 * t3818;
    t150907
}
