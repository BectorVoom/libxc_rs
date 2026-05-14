//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1194/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1194<F: Float>(t1267: F, t26975: F, t5329: F, t5341: F, t11081: F, t26960: F, t28106: F, t1856: F, t3616: F, t7772: F, t96727: F, t1851: F, t26996: F, t26966: F, t26999: F, t27042: F, t27070: F, t27077: F, t28132: F, t28137: F, t28179: F, t28204: F, t7788: F, t95909: F, t95913: F) -> (F, F, F, F) {
    let t97039 = t5329 * t26975 * t5341 * t1267;
    let t97051 = 0.7722800925925925926e-4 * t26960 * t11081 * t28106;
    let t97056 = t5329 * t26975 * t1856 * t3616;
    let t97060 = 0.92754700520833333333e-4 * t7772 * t96727;
    let t97063 = t5329 * t26996 * t1851 * t3616;
    let t97066 = 0.23214722222222222222e-2 * t95909 - 0.185671721767578125e-4 * t27077 * t97039 - 0.92754700520833333334e-4 * t28204 * t26999 - 0.92754700520833333334e-4 * t27070 * t28132 + 0.37069444444444444444e-2 * t26966 * t28179 - 0.38691203703703703704e-2 * t95913 + t97051 + 0.74203760416666666667e-3 * t27042 * t28137 - 0.13913205078125e-3 * t7772 * t97056 - t97060 - 0.34752604166666666667e-3 * t7788 * t97063;
    (t97039, t97056, t97063, t97066)
}
