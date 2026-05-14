//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1221/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1221<F: Float>(t1440: F, t2262: F, t33608: F, t1411: F, t32022: F, t32186: F, t32187: F, t32192: F, t32201: F, t33585: F, t33588: F, t33594: F, t33598: F, t33602: F, t33606: F, t9446: F, t9809: F) -> (F, F, F, F) {
    let t33609 = t2262 * t1440;
    let t33610 = t33608 * t33609;
    let t33611 = t1411 * t33610;
    let t33613 = -t32186 - 0.44218518518518518517e-2 * t32187 + 0.13402777777777777778e-2 * t32192 + 0.11054629629629629629e-2 * t32201 + 0.24872916666666666666e-2 * t33585 + 0.10416666666666666667e-1 * t9446 * t33588 - 0.27777777777777777779e-1 * t32022 * t9809 + 0.34722222222222222223e-2 * t33594 - 0.16581944444444444444e-2 * t33598 - 0.55273148148148148147e-3 * t33602 + 0.16581944444444444444e-2 * t33606 + 0.49745833333333333332e-2 * t33611;
    (t33609, t33610, t33611, t33613)
}
