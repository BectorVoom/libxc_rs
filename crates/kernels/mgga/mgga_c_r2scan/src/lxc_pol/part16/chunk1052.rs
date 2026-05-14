//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1052/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1052<F: Float>(t11780: F, t2201: F, t3602: F, t10856: F, t9423: F, t11643: F, t25983: F, t261: F, t3304: F, t9476: F, t37759: F, t37823: F, t37834: F, t37835: F, t37838: F, t37841: F, t41518: F, t41519: F) -> (F,) {
    let t43215 = t2201 * t11780 * t3602;
    let t43217 = t10856 * t9423;
    let t43219 = t25983 * t11643;
    let t43225 = t3304 * t261 * t9476;
    let t43227 = -0.59512461497092438715e-1 * t37759 + 0.43663693315433241792e-2 * t43215 - 0.48787202696913915093e-2 * t43217 - t41518 + t41519 - 0.13099107994629972538e-1 * t43219 + t37823 + t37834 + 0.29272321618148349056e-1 * t37835 + 0.22511059664845582436e0 * t37838 + 0.67533178994536747308e0 * t37841 + 0.34672886960217074253e0 * t43225;
    (t43227,)
}
