//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 957/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk957<F: Float>(t29860: F, t29901: F, t29955: F, t30016: F, t1286: F, t1310: F, t1337: F, t22872: F, t25558: F, t25869: F, t29616: F, t29727: F, t29731: F, t29736: F, t29741: F, t29745: F, t29750: F, t29756: F, t29758: F, t29790: F, t29792: F, t29794: F, t29796: F, t4415: F, t4501: F, t5501: F, t6418: F, t6562: F, t88: F, t948: F) -> (F, F) {
    let t30018 = t29860 + t29901 + t29955 + t30016;
    let t30020 = -t25558 * t6418 / 9.0 + t22872 + 2.0 / 9.0 * t5501 * t29616 - t25869 / 9.0 + 2.0 * t29727 - 2.0 / 3.0 * t1286 * t29731 - t1286 * t29736 / 3.0 + t29741 * t1310 / 6.0 - 2.0 / 3.0 * t1286 * t29745 + t1286 * t29750 - 2.0 * t948 * t6562 - t4501 * t1337 - t4415 * t1337 - 2.0 * t29756 - 4.0 * t29758 - 2.0 * t29790 - 4.0 * t29792 - 2.0 * t29794 + 4.0 * t29796 - t88 * t30018;
    (t30018, t30020)
}
