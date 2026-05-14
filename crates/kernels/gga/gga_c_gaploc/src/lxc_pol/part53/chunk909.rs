//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 909/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk909<F: Float>(t43686: F, t43693: F, t43695: F, t43698: F, t43708: F, t43716: F, t43719: F, t43721: F, t43723: F, t43726: F, t43729: F, t43735: F, t47321: F, t47325: F, t47329: F, t47331: F, t47340: F, t47341: F, t47344: F, t47347: F) -> (F,) {
    let t51126 = t47321 + t47325 - t47329 - 0.12269736305254639897e2 * t47331 + t47340 - 0.92023022289409799224e1 * t47341 - 0.38342925953920749676e0 * t47344 + 0.29792074959875355558e-1 * t47347 - t43686 + t43693 - t43695 - t43698 + t43708 - t43716 + t43719 + t43721 + t43723 + t43726 + t43729 - t43735;
    (t51126,)
}
