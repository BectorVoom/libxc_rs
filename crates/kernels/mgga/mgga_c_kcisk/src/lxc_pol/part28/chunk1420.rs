//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1420/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1420<F: Float>(t35415: F, t4419: F, t9725: F, t2804: F, t35445: F, t10000: F, t34484: F, t117841: F, t117873: F, t117876: F, t117880: F, t117887: F, t118120: F, t121219: F, t121222: F, t121226: F, t121229: F, t33183: F, t33258: F, t34496: F, t35416: F) -> (F, F) {
    let t122596 = t4419 * t35415;
    let t122597 = t9725 * t122596;
    let t122607 = t2804 * t4419 * t35445;
    let t122613 = t10000 * t34484;
    let t122616 = -0.20104166666666666667e-2 * t122597 + 0.44675925925925925927e-3 * t117841 - 0.35740740740740740741e-2 * t118120 * t34496 + t117873 + t117876 - t117880 - 0.46429444444444444444e-2 * t121219 + 0.11607361111111111111e-2 * t121222 + 0.11607361111111111111e-2 * t121226 + 0.23214722222222222222e-2 * t121229 - 0.34722222222222222223e-2 * t122607 - 0.60312500000000000001e-2 * t33258 * t35416 - 0.60312500000000000001e-2 * t33183 * t35416 + 0.34722222222222222223e-2 * t122613 + 0.23148148148148148148e-2 * t117887;
    (t122596, t122616)
}
