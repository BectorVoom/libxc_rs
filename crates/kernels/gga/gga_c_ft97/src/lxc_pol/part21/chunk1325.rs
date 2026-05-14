//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1325/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1325<F: Float>(t106875: F, t107111: F, t107113: F, t107115: F, t107117: F, t11593: F, t119473: F, t144: F, t16971: F, t16989: F, t17190: F, t17199: F, t17384: F, t1901: F, t2210: F, t23443: F, t23463: F, t23470: F, t26981: F, t30363: F, t3425: F, t4458: F, t446: F, t47659: F, t574: F, t5935: F, t64242: F, t9419: F, t95842: F, t95859: F) -> (F,) {
    let t121284 = 2.0 / 9.0 * t1901 * t106875 * t3425 - 4.0 / 3.0 * t1901 * t64242 * t26981 + t1901 * t23470 * t17199 / 9.0 + 2.0 / 9.0 * t1901 * t23443 * t17190 + 4.0 / 9.0 * t11593 * t23443 * t16989 + 2.0 / 3.0 * t446 * t144 * t119473 + t95859 + t107111 + t107113 + t107115 - 2.0 / 9.0 * t1901 * t9419 * t30363 + 4.0 / 9.0 * t47659 * t95842 * t17384 - 2.0 / 9.0 * t1901 * t2210 * t23463 * t4458 + t446 * t574 * t5935 * t16971 / 3.0 - t107117;
    (t121284,)
}
