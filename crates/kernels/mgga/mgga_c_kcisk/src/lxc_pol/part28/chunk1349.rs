//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1349/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1349<F: Float>(t121021: F, t9664: F, t34121: F, t7218: F, t34191: F, t116118: F, t116127: F, t116130: F, t116133: F, t116150: F, t121010: F, t121015: F, t121019: F, t32942: F, t32948: F, t32990: F, t35108: F, t9649: F, t9652: F) -> (F, F) {
    let t121022 = t9664 * t121021;
    let t121031 = t34121 * t7218;
    let t121034 = t34191 * t7218;
    let t121037 = 0.40208333333333333335e-2 * t9649 * t121010 + 0.49745833333333333332e-2 * t121015 - 0.16581944444444444444e-2 * t121019 + 0.34722222222222222223e-2 * t121022 - 0.20833333333333333334e-1 * t32942 * t35108 - 0.20833333333333333334e-1 * t32990 * t35108 - 0.120625e-1 * t32948 * t35108 + t116118 - t116127 + t116130 - 0.44218518518518518516e-2 * t116133 - 0.55555555555555555558e-1 * t121031 * t9652 - 0.21444444444444444445e-1 * t121034 * t9652 - t116150;
    (t121031, t121037)
}
