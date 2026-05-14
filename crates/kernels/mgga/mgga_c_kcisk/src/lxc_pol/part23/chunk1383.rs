//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1383/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1383<F: Float>(t110244: F, t110661: F, t110666: F, t110673: F, t110683: F, t113629: F, t114082: F, t114482: F, t114487: F, t114490: F, t114493: F, t114499: F, t114505: F, t114510: F, t32022: F, t33400: F, t9446: F, t9796: F) -> (F,) {
    let t114512 = -0.44218518518518518517e-2 * t114482 + 0.23148148148148148148e-2 * t110661 + 0.23148148148148148148e-2 * t110666 + 0.3684876543209876543e-2 * t114487 + 0.99491666666666666664e-2 * t114490 - t114493 + 0.62500000000000000002e-1 * t9446 * t113629 + 0.55555555555555555558e-1 * t32022 * t33400 - t114499 + 0.20833333333333333334e-1 * t9446 * t114082 - 0.15432098765432098765e-2 * t110673 - 0.33950617283950617285e-1 * t110683 + 0.24872916666666666666e-2 * t114505 + 0.10185185185185185186e0 * t110244 * t9796 - 0.33163888888888888888e-2 * t114510;
    (t114512,)
}
