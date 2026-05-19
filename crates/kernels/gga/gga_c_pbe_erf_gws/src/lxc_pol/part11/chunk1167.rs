//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1167/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1167<F: Float>(t22634: F, t22636: F, t33572: F, t22641: F, t42530: F, t18838: F, t18850: F, t18853: F, t18863: F, t18920: F, t18924: F, t18933: F, t18939: F) -> (F, F, F, F, F, F) {
    let t48495 = F::new(576.0) * t22634;
    let t48496 = F::new(960.0) * t22636;
    let t48497 = F::cast_from(0.1038945353962551798e3_f64) * t33572;
    let t48498 = F::cast_from(0.41015588084031179722e4_f64) * t22641;
    let t48499 = F::new(4.0) * t42530;
    let t48500 = -t48495 - t48496 - t18838 + t18850 + t18920 + t18924 + t18853 - t48497 - t48498 + t48499 - t18863 - t18933 + t18939;
    (t48495, t48496, t48497, t48498, t48499, t48500)
}
