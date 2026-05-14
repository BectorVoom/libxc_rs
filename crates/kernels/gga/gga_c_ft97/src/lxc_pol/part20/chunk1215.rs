//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1215/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1215<F: Float>(t24898: F, t56110: F, t29090: F, t8392: F, t10697: F, t799: F, t1508: F, t2770: F, t1882: F, t29104: F, t1212: F, t25135: F, t10261: F, t14690: F, t15255: F, t15312: F, t15369: F, t1901: F, t25368: F, t28501: F, t28506: F, t2862: F, t2867: F, t28930: F, t29072: F, t29076: F, t29185: F, t319: F, t4129: F, t4162: F, t44369: F, t446: F, t53797: F, t56127: F, t6386: F, t684: F, t7114: F, t840: F, t863: F, t871: F, t882: F, t98823: F) -> (F, F, F) {
    let t112952 = t56110 * t24898;
    let t112969 = 4.0 / 27.0 * t8392 * t29090;
    let t112975 = t799 * t10697;
    let t112987 = t2770 * t1508;
    let t112992 = 2.0 / 9.0 * t1882 * t29104;
    let t113001 = t25135 * t1212;
    let t113006 = 4.0 / 9.0 * t53797 * t112952 * t15255 - 4.0 * t1901 * t10261 * t863 * t29072 - 4.0 / 3.0 * t1901 * t56127 * t29076 + 8.0 / 81.0 * t98823 - 4.0 / 3.0 * t1901 * t15369 * t25368 * t4162 - t112969 + 2.0 / 3.0 * t446 * t840 * t871 * t6386 * t4129 + 4.0 * t1901 * t112975 * t7114 * t2867 - 4.0 / 9.0 * t1901 * t15312 * t28930 * t684 - 2.0 / 9.0 * t1901 * t44369 * t29185 - 4.0 / 9.0 * t1901 * t112987 * t14690 + t112992 + 4.0 / 3.0 * t446 * t2862 * t882 * t28501 + 4.0 / 3.0 * t446 * t2862 * t882 * t28506 + 2.0 / 3.0 * t446 * t2862 * t319 * t113001;
    (t112952, t113001, t113006)
}
