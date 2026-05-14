//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1326/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1326<F: Float>(t109717: F, t1339: F, t8171: F, t20160: F, t34706: F, t9446: F, t1286: F, t1411: F, t8015: F, t9461: F, t110663: F, t114264: F, t119019: F, t119182: F, t119186: F, t119189: F, t119194: F, t119197: F, t119203: F, t32008: F, t32019: F, t32096: F, t34693: F, t34763: F, t34803: F, t9426: F) -> (F, F, F) {
    let t119206 = t1339 * t109717 * t8171;
    let t119210 = t9446 * t20160 * t34706;
    let t119214 = t1411 * t9461 * t8015 * t1286;
    let t119218 = 0.20833333333333333334e-1 * t9446 * t119182 + 0.66327777777777777776e-2 * t119186 - 0.55273148148148148147e-2 * t119189 + 0.26805555555555555556e-2 * t110663 * t34763 + 0.16083333333333333334e-1 * t32008 * t119194 - 0.69444444444444444447e-2 * t119197 - 0.24125e-1 * t9426 * t119019 - 0.46296296296296296297e-2 * t32019 * t34803 - 0.22109259259259259259e-2 * t119203 + 0.88437037037037037035e-2 * t119206 - 0.23148148148148148149e-2 * t114264 + 0.34722222222222222223e-2 * t119210 - 0.55273148148148148147e-3 * t119214 - 0.20833333333333333334e-1 * t32096 * t34693;
    (t119206, t119214, t119218)
}
