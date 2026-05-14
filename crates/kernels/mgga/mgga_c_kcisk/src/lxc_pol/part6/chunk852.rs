//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 852/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk852<F: Float>(t776: F, t12169: F, t28368: F, t10832: F, t28532: F, t41: F, t28800: F, t7568: F, t2442: F, t2620: F, t29275: F, t29282: F, t525: F, t642: F, t7567: F, t773: F, t8781: F, t8787: F, t9192: F) -> (F, F) {
    let t777 = t776 < -0.66725e-1;
    let t29890 = t12169 * t28368;
    let t29891 = t10832 * t29890;
    let t29894 = t28532 * t41;
    let t29910 = t7568 * t28800;
    let t29917 = piecewise3(t777, 0.0, 10.0 / 9.0 * t525 * t29894 * t642 - 10.0 / 9.0 * t525 * t9192 * t2442 + 40.0 / 27.0 * t525 * t2620 * t8781 - 10.0 / 9.0 * t525 * t2620 * t8787 - 280.0 / 243.0 * t525 * t773 * t29275 + 40.0 / 27.0 * t7567 * t29910 - 10.0 / 27.0 * t525 * t773 * t29282);
    (t29891, t29917)
}
