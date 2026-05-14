//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1106/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1106<F: Float>(t1020: F, t18530: F, t7718: F, t1856: F, t26996: F, t5329: F, t5336: F, t1267: F, t30066: F, t6774: F, t26975: F, t5341: F, t100162: F, t100170: F, t27014: F, t27028: F, t27077: F, t28132: F, t28179: F, t28190: F, t28204: F, t29161: F, t68040: F, t68045: F, t7788: F, t92787: F, t93050: F) -> (F, F, F, F, F) {
    let t100229 = t1020 * t7718 * t18530;
    let t100235 = t5329 * t26996 * t5336 * t1856;
    let t100244 = t5329 * t30066 * t6774 * t1267;
    let t100257 = t5329 * t26975 * t1856 * t5341;
    let t100262 = -0.13901041666666666667e-2 * t28190 * t28179 - 0.25794135802469135802e-3 * t100229 - 0.69505208333333333334e-3 * t27014 * t29161 - 0.69505208333333333334e-3 * t7788 * t100235 + 0.208515625e-2 * t7788 * t5329 * t92787 * t68045 + 0.69505208333333333334e-3 * t7788 * t100244 - 0.13901041666666666667e-2 * t7788 * t5329 * t27028 * t68040 - 0.69505208333333333334e-3 * t7788 * t100170 - 0.92754700520833333334e-4 * t28204 * t28132 - 0.185671721767578125e-4 * t27077 * t100257 + 0.24777891269883300782e-5 * t93050 * t100162;
    (t100229, t100235, t100244, t100257, t100262)
}
