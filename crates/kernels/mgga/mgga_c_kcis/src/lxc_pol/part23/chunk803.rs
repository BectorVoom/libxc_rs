//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 803/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk803<F: Float>(t11479: F, t11482: F, t16195: F, t16198: F, t16201: F, t16204: F, t16207: F, t16210: F, t16213: F, t16215: F, t16219: F, t11475: F, t16046: F, t16052: F, t16057: F, t16067: F, t16071: F, t16075: F, t16080: F, t16084: F, t16127: F, t16129: F, t16132: F, t16135: F, t16137: F, t16142: F, t16146: F, t16163: F, t16165: F, t16168: F, t16292: F, t16306: F) -> (F,) {
    let t16328 = 0.258925e1 * t16195 + 0.16557e0 * t16198 - 0.49671e0 * t16201 - 0.27595e-1 * t16204 - 0.36793333333333333333e-1 * t16207 + 0.11038e0 * t16210 + 0.16557e0 * t16213 + 0.16504875e0 * t16215 - t11479 - t11482 + 0.16557e0 * t16219;
    let t16330 = -0.71747e0 * t16127 - 0.91983333333333333334e-1 * t16129 - 0.412621875e-1 * t16132 - 0.258925e1 * t16135 - 0.1294625e1 * t16137 - 0.22141166666666666666e1 * t16052 - 0.13418888888888888889e0 * t16046 - 0.66228e0 * t16142 - t16292 + 0.36793333333333333334e-1 * t16146 + t16306 + 0.16504875e0 * t16163 + 0.82524375e-1 * t16165 + 0.19419375e1 * t16168 - 0.33547222222222222222e0 * t16057 + 0.80513333333333333333e0 * t16067 - 0.20128333333333333333e0 * t16071 - 0.181155e1 * t16075 - 0.24154e1 * t16080 + 0.60385e0 * t16084 - 0.11038e0 * t11475 + t16328;
    (t16330,)
}
