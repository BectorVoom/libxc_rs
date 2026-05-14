//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 995/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk995<F: Float>(t3739: F, t7833: F, t1286: F, t3485: F, t7710: F, t3484: F, t3482: F, t19735: F, t19738: F, t26506: F, t26510: F, t26514: F, t26518: F, t26753: F, t26755: F, t26758: F, t26762: F, t26764: F, t26767: F, t26770: F, t26776: F, t26780: F, t26783: F, t26785: F) -> (F, F, F, F) {
    let t26787 = t3739 * t7833;
    let t26790 = t3485 * t7710 * t1286;
    let t26791 = t3484 * t26790;
    let t26792 = t3482 * t26791;
    let t26794 = 0.33163888888888888888e-2 * t26506 - 0.33163888888888888888e-2 * t26510 + 0.27636574074074074073e-2 * t26514 - 0.22109259259259259259e-2 * t26518 - 0.24872916666666666666e-2 * t26753 - 0.16581944444444444444e-2 * t26755 - 0.33163888888888888888e-2 * t26758 + 0.66327777777777777776e-2 * t26762 - 0.22109259259259259259e-2 * t26764 + 0.99491666666666666664e-2 * t26767 - 0.33163888888888888888e-2 * t26770 + 0.44218518518518518516e-2 * t19735 + t19738 + 0.11054629629629629629e-2 * t26776 + 0.33163888888888888888e-2 * t26780 + 0.13265555555555555555e-1 * t26783 + 0.22109259259259259259e-2 * t26785 + 0.11054629629629629629e-2 * t26787 + 0.11054629629629629629e-2 * t26792;
    (t26787, t26790, t26792, t26794)
}
