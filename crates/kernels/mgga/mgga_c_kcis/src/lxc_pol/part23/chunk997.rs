//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 997/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk997<F: Float>(t28549: F, t7909: F, t3984: F, t2237: F, t27483: F, t27486: F, t28501: F, t28506: F, t28508: F, t28511: F, t28514: F, t28517: F, t28520: F, t28522: F, t28526: F, t28529: F, t28532: F, t28535: F, t28544: F, t28547: F, t7895: F, t7901: F, t7908: F, t7916: F, t8151: F, t8159: F) -> (F, F, F) {
    let t28550 = t7909 * t28549;
    let t28551 = t3984 * t28550;
    let t28554 = 0.16581944444444444444e-2 * t28501 + 0.49745833333333333332e-2 * t28506 - 0.44218518518518518517e-2 * t28508 + 0.11054629629629629629e-2 * t28511 - 0.33163888888888888888e-2 * t28514 + 0.27636574074074074073e-2 * t28517 - 0.16581944444444444444e-2 * t28520 - t27483 + t27486 + 0.23168402777777777778e-3 * t28522 - 0.24872916666666666666e-2 * t28526 + 0.16581944444444444444e-2 * t28529 - 0.24872916666666666666e-2 * t28532 + 0.69505208333333333333e-3 * t2237 * t28535 + 0.69505208333333333333e-3 * t7895 * t8159 - 0.18534722222222222222e-2 * t8151 * t7916 - 0.18534722222222222222e-2 * t8151 * t7901 - 0.24734586805555555555e-3 * t28544 * t7901 - 0.16581944444444444444e-2 * t28547 + 0.23168402777777777778e-3 * t7908 * t28551;
    (t28550, t28551, t28554)
}
