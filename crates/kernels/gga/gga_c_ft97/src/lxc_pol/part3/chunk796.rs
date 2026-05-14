//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 796/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk796<F: Float>(t18127: F, t200: F, t236: F, t4977: F, t3724: F, t13443: F, t17993: F, t17994: F, t17997: F, t18003: F, t18007: F, t18012: F, t18015: F, t18018: F, t18021: F, t18024: F, t18084: F, t18090: F, t224: F, t2387: F, t3723: F, t3789: F, t4986: F, t678: F, t680: F, t690: F, t695: F, t709: F, t710: F) -> (F,) {
    let t18128 = t18127 * t200;
    let t18132 = t236 * t4977;
    let t18133 = t3724 * t18132;
    let t18136 = -0.2370952259137005195e-1 * t17993 * t17994 - 6.0 * t3789 * t17997 * t709 + 0.2370952259137005195e-1 * t13443 * t18003 + 0.11627450473218896e-1 * t2387 * t18007 - 0.32253953169881963531e-5 * t678 * t18012 + 0.23254900946437792e-2 * t678 * t18015 - 0.279058811357253504e-2 * t678 * t18018 + 0.46509801892875584e-2 * t678 * t18021 - 0.11619434043764639964e-3 * t678 * t18024 - t224 * t695 * t18084 - 2.0 * t4986 * t710 - 0.23254900946437792e-1 * t18090 * t690 - 0.11627450473218896e-1 * t678 * t680 * t18128 + 0.67598802253579164263e-4 * t3723 * t18133;
    (t18136,)
}
