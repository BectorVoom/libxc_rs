//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1193/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1193<F: Float>(t3797: F, t7710: F, t9461: F, t1339: F, t468: F, t8082: F, t415: F, t2718: F, t32186: F, t33373: F, t33377: F, t33384: F, t33594: F, t34744: F, t34795: F, t34799: F, t34803: F, t34807: F, t34811: F, t9426: F, t9446: F, t9796: F, t9805: F) -> (F, F, F, F, F, F) {
    let t34815 = t3797 * t7710;
    let t34816 = t9461 * t34815;
    let t34817 = t1339 * t34816;
    let t34826 = t468 * t8082;
    let t34827 = t415 * t34826;
    let t34829 = -0.10416666666666666667e-1 * t34795 * t2718 - 0.34722222222222222223e-2 * t9446 * t34799 - 0.46296296296296296297e-2 * t9446 * t34803 + 0.33163888888888888888e-2 * t34807 - 0.33163888888888888888e-2 * t34811 - t32186 - 0.8041666666666666667e-2 * t9426 * t34744 + 0.16581944444444444444e-2 * t34817 - 0.69444444444444444446e-2 * t33384 * t9805 + 0.20833333333333333334e-1 * t33373 * t9796 + 0.8041666666666666667e-2 * t33377 * t9796 + 0.69444444444444444446e-2 * t33594 - 0.55273148148148148147e-3 * t34827;
    (t34815, t34816, t34817, t34826, t34827, t34829)
}
